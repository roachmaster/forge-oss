//! Pre-render pipeline stage.
//! For now, this is just a ceremonial placeholder.

/// Run any pre-render checks or transformations.
/// MVP: just logs that the pre-render pipeline is running.
pub fn run(values_path: &str, template_path: &str) {
    println!(
        "[forgery-codegen::pipeline] pre_render_pipeline running for values='{values_path}', template='{template_path}'"
    );
}
