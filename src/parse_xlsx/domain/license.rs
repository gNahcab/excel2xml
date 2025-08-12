use crate::parse_xlsx::errors::ExcelDataError;

#[derive(Debug)]
pub enum  License {
    CCBYNCSA40,
    CCBYNC40,
    CCBYSA40,
    CCBY40,
    CCBYND40,
    PUBLICDOMAIN,
    UNKNOWNLICENSE,
    AIGENERATED,
    CCBYNCND40,
}
impl License {
    pub fn rdfh_str(&self) ->  &'static str {
     match self {
         License::CCBYNCSA40 => {
             "http://rdfh.ch/licenses/cc-by-nc-sa-4.0"
         }
         License::CCBYNC40 => {
             "http://rdfh.ch/licenses/cc-by-nc-4.0"
         }
         License::CCBYSA40 => {
             "http://rdfh.ch/licenses/cc-by-sa-4.0"
         }
         License::CCBY40 => {
             "http://rdfh.ch/licenses/cc-by-4.0"
         }
         License::CCBYND40 => {
             "http://rdfh.ch/licenses/cc-by-nd-4.0"
         }
         License::CCBYNCND40 => {
             "http://rdfh.ch/licenses/cc-by-nc-nd-4.0"
         }
         License::PUBLICDOMAIN => {
             "http://rdfh.ch/licenses/public-domain"
         }
         License::UNKNOWNLICENSE => {
             "http://rdfh.ch/licenses/unknown"
         }
         License::AIGENERATED => {
             "http://rdfh.ch/licenses/ai-generated"
         }
     }
}

}

pub fn to_license(value: &String) -> Result<License, ExcelDataError> {
    match value.as_str() {
        "CC BY 4.0" | "http://rdfh.ch/licenses/cc-by-4.0" |
        "https://creativecommons.org/licenses/by/4.0/" => {
            Ok(License::CCBY40)
        },
        "CC BY-SA 4.0" | "http://rdfh.ch/licenses/cc-by-sa-4.0" |
        "https://creativecommons.org/licenses/by-sa/4.0/" => {
            Ok(License::CCBYSA40)
        },
        "CC BY-NC 4.0" | "http://rdfh.ch/licenses/cc-by-nc-4.0" |
        "https://creativecommons.org/licenses/by-nc/4.0/" => {
            Ok(License::CCBYNC40)
        },
        "CC BY-NC-SA 4.0" | "http://rdfh.ch/licenses/cc-by-nc-sa-4.0" |
        "https://creativecommons.org/licenses/by-nc-sa/4.0/" => {
            Ok(License::CCBYNCSA40)
        },
        "CC BY-ND 4.0" | "http://rdfh.ch/licenses/cc-by-nd-4.0" |
        "https://creativecommons.org/licenses/by-nd/4.0/" => {
            Ok(License::CCBYND40)
        },
        "CC BY-NC-ND 4.0" | "http://rdfh.ch/licenses/cc-by-nc-nd-4.0" |
        "https://creativecommons.org/licenses/by-nc-nd/4.0/" => {
            Ok(License::CCBYNCND40)
        },
        "AI-Generated Content - Not Protected by Copyright" |
        "http://rdfh.ch/licenses/ai-generated" => {
            Ok(License::AIGENERATED)
        }
        "Unknown License - Ask Copyright Holder for Permission" | "http://rdfh.ch/licenses/unknown"
        => {
            Ok(License::UNKNOWNLICENSE)
        }
        "Public Domain - Not Protected by Copyright" | "http://rdfh.ch/licenses/public-domain" => {
            Ok(License::PUBLICDOMAIN)
        }
        _ => {
            Err(ExcelDataError::InputError(format!(")Unknown license: {}, add or change first.", value)))
        }
    }
}