    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
use imara_diff::{intern::InternedInput, Algorithm, UnifiedDiffBuilder};
    /// Old commit, default is HEAD
    #[clap(long, value_name = "COMMIT")]
    /// New commit, default is working directory
    #[clap(long, value_name = "COMMIT")]
    /// Use stage as new commit. This option is conflict with --new.
    #[clap(long)]
    // Print the result to file
    #[clap(long, value_name = "FILENAME")]
                // NOTE: git didn't show diff for untracked files, but we do
    let paths: Vec<PathBuf> = args.pathspec.iter().map(util::to_workdir_path).collect();
                #[cfg(unix)]
                let mut child = Command::new("less")
                    .arg("-R")
                    .arg("-F")
                    .stdin(Stdio::piped())
                    .spawn()
                    .expect("failed to execute process");
    let new_blobs: HashMap<PathBuf, SHA1> = new_blobs.into_iter().collect();
    // unison set
    let union_files: HashSet<PathBuf> = old_blobs.keys().chain(new_blobs.keys()).cloned().collect();
    tracing::debug!(
        "old blobs {:?}, new blobs {:?}, union files {:?}",
        old_blobs.len(),
        new_blobs.len(),
        union_files.len()
    );

            Ok(blob) => blob.data,
                std::fs::read(&file)

    for file in union_files {
        if !filter.is_empty() && !filter.iter().any(|path| file.sub_of(path)) {

        let new_hash = new_blobs.get(&file);
        let old_hash = old_blobs.get(&file);
        if new_hash == old_hash {
            continue;
        }

        let old_content = match old_hash.as_ref() {
            Some(hash) => read_content(&file, hash),
            None => Vec::new(),
        };
        let new_content = match new_hash.as_ref() {
            Some(hash) => read_content(&file, hash),
            None => Vec::new(),
        };

        writeln!(
            w,
            "diff --git a/{} b/{}",
            file.display(),
            file.display() // files name is always the same, current did't support rename
        )
        .unwrap();

        if old_hash.is_none() {
            writeln!(w, "new file mode 100644").unwrap();
        } else if new_hash.is_none() {
            writeln!(w, "deleted file mode 100644").unwrap();
        }

        let old_index = old_hash.map_or("0000000".to_string(), |h| h.to_string()[0..8].to_string());
        let new_index = new_hash.map_or("0000000".to_string(), |h| h.to_string()[0..8].to_string());
        writeln!(w, "index {}..{}", old_index, new_index).unwrap();
        // check is the content is valid utf-8 or maybe binary
        let old_type = infer::get(&old_content);
        let new_type = infer::get(&new_content);
        match (
            String::from_utf8(old_content),
            String::from_utf8(new_content),
        ) {
            (Ok(old_text), Ok(new_text)) => {
                imara_diff_result(&old_text, &new_text, w);
            }
            _ => {
                // TODO: Handle non-UTF-8 data as binary for now; consider optimization in the future.
                    "Binary files a/{} and b/{} differ",
                    file_display(&file, old_hash, old_type),
                    file_display(&file, new_hash, new_type)
// display file with type
fn file_display(file: &Path, hash: Option<&SHA1>, file_type: Option<infer::Type>) -> String {
    let file_name = match hash {
        Some(_) => file.display().to_string(),
        None => "dev/null".to_string(),
    };

    if let Some(file_type) = file_type {
        // Check if the file type is displayable in browser, like image, audio, video, etc.
        if matches!(
            file_type.matcher_type(),
            infer::MatcherType::Audio | infer::MatcherType::Video | infer::MatcherType::Image
        ) {
            return format!("{} ({})", file_name, file_type.mime_type()).to_string();
        }
    }
    file_name
}

#[allow(dead_code)]
fn similar_diff_result(old: &str, new: &str, w: &mut dyn io::Write) {
fn imara_diff_result(old: &str, new: &str, w: &mut dyn io::Write) {
    let input = InternedInput::new(old, new);
    let diff = imara_diff::diff(
        Algorithm::Histogram,
        &input,
        UnifiedDiffBuilder::new(&input),
    );
    write!(w, "{}", diff).unwrap();
}

    fn test_similar_diff_result() {
        similar_diff_result(old, new, &mut buf);