use syn::{parse::Parse, parse_macro_input, Token};

#[derive(Debug)]
enum MemoryRegionAttribute {
    Address(syn::LitInt),
    Size(syn::Expr),
    Access(syn::LitStr),
}

impl Parse for MemoryRegionAttribute {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        match input.parse::<syn::Ident>() {
            Ok(ident) if ident == "address" => {
                let _: Token![=] = input.parse()?;
                let name = input.parse()?;
                Ok(Self::Address(name))
            }
            Ok(ident) if ident == "size" => {
                let _: Token![=] = input.parse()?;
                let name = input.parse()?;
                Ok(Self::Size(name))
            }
            Ok(ident) if ident == "access" => {
                let _: Token![=] = input.parse()?;
                let name = input.parse()?;
                Ok(Self::Access(name))
            }
            Ok(ident) => {
                let message = format!("Unknown memory region attribute with name `{}`", ident);
                Err(syn::Error::new(ident.span(), message))
            }
            Err(err) => {
                let message = format!("Expected identifier. {}", err);
                Err(syn::Error::new(err.span(), message))
            }
        }
    }
}

#[derive(Debug)]
struct MemoryRegion {
    name: syn::Ident,
    attributes: syn::punctuated::Punctuated<MemoryRegionAttribute, Token![,]>,
}

impl Parse for MemoryRegion {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = input.parse::<syn::Ident>()?;
        let _: Token![=>] = input.parse()?;
        let content;
        let _ = syn::braced!(content in input);

        let attributes: syn::punctuated::Punctuated<MemoryRegionAttribute, Token![,]> =
            content.parse_terminated(MemoryRegionAttribute::parse)?;

        Ok(MemoryRegion { name, attributes })
    }
}

#[derive(Debug)]
enum SectionAttribute {
    Region(syn::Ident),
    Offset(syn::LitInt),
    Size(syn::Expr),
    Vma(syn::Ident),
    Lma(syn::Ident),
}

impl Parse for SectionAttribute {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        match input.parse::<syn::Ident>() {
            Ok(ident) if ident == "region" => {
                let _: Token![=] = input.parse()?;
                let name = input.parse()?;
                Ok(Self::Region(name))
            }
            Ok(ident) if ident == "offset" => {
                let _: Token![=] = input.parse()?;
                let name = input.parse()?;
                Ok(Self::Offset(name))
            }
            Ok(ident) if ident == "size" => {
                let _: Token![=] = input.parse()?;
                let name = input.parse()?;
                Ok(Self::Size(name))
            }
            Ok(ident) if ident == "vma" => {
                let _: Token![=] = input.parse()?;
                let name = input.parse()?;
                Ok(Self::Vma(name))
            }
            Ok(ident) if ident == "lma" => {
                let _: Token![=] = input.parse()?;
                let name = input.parse()?;
                Ok(Self::Lma(name))
            }
            Ok(ident) => {
                let message = format!("Unknown section attribute with name `{}`", ident);
                Err(syn::Error::new(ident.span(), message))
            }
            Err(err) => {
                let message = format!("Expected identifier. {}", err);
                Err(syn::Error::new(err.span(), message))
            }
        }
    }
}

#[derive(Debug)]
struct Section {
    name: syn::Ident,
    attributes: syn::punctuated::Punctuated<SectionAttribute, Token![,]>,
}

impl Parse for Section {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // Parse and validate the identifier
        let name = match input.parse::<syn::Ident>()? {
            ident if ident == "Text" => ident,
            ident if ident == "Data" => ident,
            ident if ident == "Bss" => ident,
            ident if ident == "CcramData" => ident,
            ident if ident == "CcramBss" => ident,
            ident if ident == "VectorTable" => ident,
            ident if ident == "Ramfunc" => ident,
            ident => {
                let message = format!("{} is not a valid `Section` identifier", ident);
                return Err(syn::Error::new(ident.span(), message));
            }
        };

        let _: Token![=>] = input.parse()?;
        let content;
        let _ = syn::braced!(content in input);

        let attributes: syn::punctuated::Punctuated<SectionAttribute, Token![,]> =
            content.parse_terminated(SectionAttribute::parse)?;

        Ok(Section { name, attributes })
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
