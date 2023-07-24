pub struct XboxLiveLanguage<'a>{
    name: &'a str, //"Argentina"
    short_id: &'a str, //"AR"
    identifier: &'a str, //"es_AR"
    local: &'a str, //"es-AR"
}

pub const ARGENTINA: XboxLiveLanguage = XboxLiveLanguage::new("Argentina", "AR", "es_AR", "es-AR");
pub const AUSTRALIA: XboxLiveLanguage = XboxLiveLanguage::new("Australia", "AU", "en_AU", "en-AU") ;
pub const AUSTRIA: XboxLiveLanguage = XboxLiveLanguage::new("Austria", "AT", "de_AT", "de-AT");
pub const BELGIUM: XboxLiveLanguage = XboxLiveLanguage::new("Belgium", "BE", "fr_BE", "fr-BE");
pub const BELGIUM_NL: XboxLiveLanguage = XboxLiveLanguage::new("Belgium (NL)", "NL", "nl_BE", "nl-BE");
pub const BRAZIL: XboxLiveLanguage = XboxLiveLanguage::new("Brazil", "BR", "pt_BR", "pt-BR");
pub const CANADA: XboxLiveLanguage = XboxLiveLanguage::new("Canada", "CA", "en_CA", "en-CA");
pub const CANADA_FR: XboxLiveLanguage = XboxLiveLanguage::new("Canada (FR)", "CA", "fr_CA", "fr-CA");
pub const CZECH_REPUBLIC: XboxLiveLanguage = XboxLiveLanguage::new("Czech Republic", "CZ", "en_CZ", "en-CZ");
pub const DENMARK: XboxLiveLanguage = XboxLiveLanguage::new("Denmark", "DK", "da_DK", "da-DK");
pub const FINLAND: XboxLiveLanguage = XboxLiveLanguage::new("Finland", "FI", "fi_FI", "fi-FI");
pub const FRANCE: XboxLiveLanguage = XboxLiveLanguage::new("France", "FR", "fr_FR", "fr-FR");
pub const GERMANY: XboxLiveLanguage = XboxLiveLanguage::new("Germany", "DE", "de_DE", "de-DE");
pub const GREECE: XboxLiveLanguage = XboxLiveLanguage::new("Greece", "GR", "en_GR", "en-GR");
pub const HONG_KONG: XboxLiveLanguage = XboxLiveLanguage::new("Hong Kong", "HK", "en_HK", "en-HK");
pub const HUNGARY: XboxLiveLanguage = XboxLiveLanguage::new("Hungary", "HU", "en_HU", "en-HU");
pub const INDIA: XboxLiveLanguage = XboxLiveLanguage::new("India", "IN", "en_IN", "en-IN");
pub const GREAT_BRITAIN: XboxLiveLanguage = XboxLiveLanguage::new("Great Britain", "GB", "en_GB", "en-GB");
pub const ITALY: XboxLiveLanguage = XboxLiveLanguage::new("Italy", "IT", "it_IT", "it-IT");
pub const JAPAN: XboxLiveLanguage = XboxLiveLanguage::new("Japan", "JP", "ja_JP", "ja-JP");
pub const MEXICO: XboxLiveLanguage = XboxLiveLanguage::new("Mexico", "MX", "es_MX", "es-MX");
pub const CHILE: XboxLiveLanguage = XboxLiveLanguage::new("Chile", "CL", "es_CL", "es-CL");
pub const COLOMBIA: XboxLiveLanguage = XboxLiveLanguage::new("Colombia", "CO", "es_CO", "es-CO");
pub const NETHERLANDS: XboxLiveLanguage = XboxLiveLanguage::new("Netherlands", "NL", "nl_NL", "nl-NL");
pub const NEW_ZEALAND: XboxLiveLanguage = XboxLiveLanguage::new("New Zealand", "NZ", "en_NZ", "en-NZ");
pub const NORWAY: XboxLiveLanguage = XboxLiveLanguage::new("Norway", "NO", "nb_NO", "nb-NO");
pub const POLAND: XboxLiveLanguage = XboxLiveLanguage::new("Poland", "PL", "pl_PL", "pl-PL");
pub const PORTUGAL: XboxLiveLanguage = XboxLiveLanguage::new("Portugal", "PT", "pt_PT", "pt-PT");
pub const RUSSIA: XboxLiveLanguage = XboxLiveLanguage::new("Russia", "RU", "ru_RU", "ru-RU");
pub const SAUDI_ARABIA: XboxLiveLanguage = XboxLiveLanguage::new("Saudi Arabia", "SA", "en_SA", "en-SA");
pub const SINGAPORE: XboxLiveLanguage = XboxLiveLanguage::new("Singapore", "SG", "en_SG", "en-SG");
pub const SLOVAKIA: XboxLiveLanguage = XboxLiveLanguage::new("Slovakia", "SK", "en_SK", "en-SK");
pub const SOUTH_AFRICA: XboxLiveLanguage = XboxLiveLanguage::new("South Africa", "ZA", "en_ZA", "en-ZA");
pub const KOREA: XboxLiveLanguage = XboxLiveLanguage::new("Korea", "KR", "ko_KR", "ko-KR");
pub const SPAIN: XboxLiveLanguage = XboxLiveLanguage::new("Spain", "ES", "es_ES", "es-ES");
pub const SWITZERLAND: XboxLiveLanguage = XboxLiveLanguage::new("Switzerland", "CH", "de_CH", "de-CH");
pub const SWITZERLAND_FR: XboxLiveLanguage = XboxLiveLanguage::new("Switzerland (FR)", "CH", "fr_CH", "fr-CH");
pub const UNITED_ARAB_EMIRATES: XboxLiveLanguage = XboxLiveLanguage::new("United Arab Emirates", "AE", "en_AE", "en-AE");
pub const UNITED_STATES: XboxLiveLanguage = XboxLiveLanguage::new("United States", "US", "en_US", "en-US");
pub const IRELAND: XboxLiveLanguage = XboxLiveLanguage::new("Ireland", "IE", "en_IE", "en-IE");



impl XboxLiveLanguage <'static> {
    const fn new(name: & 'static str , short_id: & 'static str, identifier: & 'static str, local: & 'static str ) -> Self{
        XboxLiveLanguage{
            name,
            short_id,
            identifier,
            local,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn short_id(&self) -> &str {
        &self.short_id
    }
    pub fn identifier(&self) -> &str {
        &self.identifier
    }
    pub fn local(&self) -> &str {
        &self.local
    }
}




pub const MARKETS: phf::Map<&'static str, XboxLiveLanguage> = phf::phf_map!{
    "AR" => ARGENTINA,
    "AU" => AUSTRALIA,
    "AT" => AUSTRIA,
    "BE" => BELGIUM,
    "NE" => BELGIUM_NL,
    "BR" => BRAZIL,
    "CA" => CANADA,
    "CZ" => CZECH_REPUBLIC,
    "DK" => DENMARK,
    "FI" => FINLAND,
    "FR" => FRANCE,
    "DE" => GERMANY,
    "GR" => GREECE,
    "HK" => HONG_KONG,
    "HU" => HUNGARY,
    "IN" => INDIA,
    "GB" => GREAT_BRITAIN,
    "IT" => ITALY,
    "JP" => JAPAN,
    "MX" => MEXICO,
    "CL" => CHILE,
    "CO" => COLOMBIA,
    "NL" => NETHERLANDS,
    "NZ" => NEW_ZEALAND,
    "NO" => NORWAY,
    "PL" => POLAND,
    "PT" => PORTUGAL,
    "RU" => RUSSIA,
    "SA" => SAUDI_ARABIA,
    "SG" => SINGAPORE,
    "SK" => SLOVAKIA,
    "ZA" => SOUTH_AFRICA,
    "KR" => KOREA,
    "ES" => SPAIN,
    "CH" => SWITZERLAND,
    "AE" => UNITED_ARAB_EMIRATES,
    "US" => UNITED_STATES,
    "IE" => IRELAND,
};

pub const LANGUAGES: phf::Map<&'static str, XboxLiveLanguage> = phf::phf_map!{
    "fr" => FRANCE,
    "en" => GREAT_BRITAIN,
    "nl" => NETHERLANDS,
    "es" => SPAIN,
    "de" => GERMANY,
    "pt" => PORTUGAL,
    "da" => DENMARK,
    "fi" => FINLAND,
    "it" => ITALY,
    "ja" => JAPAN,
    "nb" => NORWAY,
    "ru" => RUSSIA,
    "ko" => KOREA,
};