use makepad_widgets::{ActionDefaultRef, DefaultNone};

#[derive(Clone, Debug, DefaultNone)]
pub enum BackendResult {
    AnalysisComplete {
        lunch_summary: String,
        lunch_details: String,
        dinner_summary: String,
        dinner_details: String,
        exception_summary: String,
        exception_details: String,
    },
    ConfigLoaded(String),
    ConfigSaved,
    None,
}
