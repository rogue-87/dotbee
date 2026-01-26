pub struct Icons {
    pub check: String,
    pub cross: String,
    pub link: String,
    pub unlink: String,
    pub warning: String,
    pub info: String,
    pub error: String,
}

impl Icons {
    pub fn new(style: &str) -> Self {
        match style.to_lowercase().as_str() {
            #[rustfmt::skip]
            "emoji" => Self {
                check   : "✅ ".to_string(),
                cross   : "❌ ".to_string(),
                link    : "🔗 ".to_string(),
                unlink  : "💔 ".to_string(),
                warning : "⚠️ ".to_string(),
                info    : "ℹ️ ".to_string(),
                error   : "🚫 ".to_string(),
            },
            #[rustfmt::skip]
            "nerdfont" => Self {
                check   : " ".to_string(),
                cross   : " ".to_string(),
                link    : " ".to_string(),
                unlink  : " ".to_string(),
                warning : " ".to_string(),
                info    : " ".to_string(),
                error   : " ".to_string(),
            },
            // default
            #[rustfmt::skip]
            "text" | _ => Self {
                check   : "DONE  ".to_string(),
                cross   : "FAIL  ".to_string(),
                link    : "LINK  ".to_string(),
                unlink  : "MISS  ".to_string(),
                warning : "WARN  ".to_string(),
                info    : "INFO  ".to_string(),
                error   : "ERROR ".to_string(),
            },
        }
    }
}
