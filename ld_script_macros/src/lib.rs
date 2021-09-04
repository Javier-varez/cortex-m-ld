use syn::{parse::Parse, parse_macro_input, Token};

#[derive(Debug)]
struct MemoryRegion {
    name: String,
}

impl Parse for MemoryRegion {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name: syn::Ident = input.parse()?;
        let _: Token![=>] = input.parse()?;
        let _content;
        let _ = syn::braced!(_content in input);

        Ok(MemoryRegion {
            name: name.to_string(),
        })
    }
}

#[derive(Debug)]
struct Section {
    name: String,
}

impl Parse for Section {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name: syn::Ident = input.parse()?;
        let _: Token![=>] = input.parse()?;
        let _content;
        let _ = syn::braced!(_content in input);

        Ok(Section {
            name: name.to_string(),
        })
    }
}

#[derive(Debug)]
struct MemoryRegions {
    ident: syn::Ident,
    regions: syn::punctuated::Punctuated<MemoryRegion, Token![,]>,
}

impl Parse for MemoryRegions {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident: syn::Ident = input.parse()?;
        let _: Token![=>] = input.parse()?;
        let content;
        let _ = syn::braced!(content in input);

        let regions = content.parse_terminated(MemoryRegion::parse)?;
        Ok(MemoryRegions { ident, regions })
    }
}

#[derive(Debug)]
struct Sections {
    ident: syn::Ident,
    sections: syn::punctuated::Punctuated<Section, Token![,]>,
}

impl Parse for Sections {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident: syn::Ident = input.parse()?;
        let _: Token![=>] = input.parse()?;
        let content;
        let _ = syn::braced!(content in input);

        let sections = content.parse_terminated(Section::parse)?;
        Ok(Sections { ident, sections })
    }
}

#[derive(Debug)]
enum Parts {
    MemoryRegions(MemoryRegions),
    Sections(Sections),
}

impl Parse for Parts {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let forked = input.fork();
        let ident = forked.parse::<syn::Ident>()?;

        if ident == "MemoryRegions" {
            let regions = input.parse::<MemoryRegions>()?;
            Ok(Parts::MemoryRegions(regions))
        } else if ident == "Sections" {
            let sections = input.parse::<Sections>()?;
            Ok(Parts::Sections(sections))
        } else {
            Err(input.error("Expected either `MemoryRegions` or `Sections`"))
        }
    }
}

#[derive(Debug)]
struct LinkerScript {
    parts: syn::punctuated::Punctuated<Parts, Token![,]>,
}

impl Parse for LinkerScript {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<LinkerScript> {
        let parts: syn::punctuated::Punctuated<Parts, Token![,]> =
            input.parse_terminated(Parts::parse)?;

        let mut found_sections = false;
        let mut found_memory_regions = false;

        for part in &parts {
            match part {
                Parts::MemoryRegions(regions) => {
                    if found_memory_regions {
                        return Err(syn::Error::new(
                            regions.ident.span(),
                            "More than one `MemoryRegions` element found",
                        ));
                    }
                    found_memory_regions = true
                }
                Parts::Sections(sections) => {
                    if found_sections {
                        return Err(syn::Error::new(
                            sections.ident.span(),
                            "More than one `Sections` element found",
                        ));
                    }
                    found_sections = true
                }
            }
        }

        if !found_memory_regions {
            return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "`MemoryRegions` is a required field",
            ));
        }

        if !found_sections {
            return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "`Sections` is a required field",
            ));
        }

        Ok(LinkerScript { parts })
    }
}

#[proc_macro]
pub fn linker_script(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(item as LinkerScript);

    eprintln!("{:#?}", ast);

    proc_macro::TokenStream::new()
}
