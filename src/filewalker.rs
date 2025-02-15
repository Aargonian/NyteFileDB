/**
 * FileWalker represents the process of actually walking the tree of files we will be processing.
 * It enables a structure not only for actually walking the tree, but for injecting hooks into the
 * walking process.
 */
//TODO: Implement "IntoIterator" for FileWalker
//TODO: Implement "Iterator?" for FileWalker
//Desired Iterator Featuers:
//    - Collect to do eager evaluation of the whole tree
struct FileWalker {
    current_dir: Option<&Path>,
    current_item: Option<&Path>,
}

impl <'a> Iterator for FileWalker<'a> {
    type Item = &'a Path;

    fn next(&mut self) -> Option<Self::Item> {
        let current_dir: &Path = self.current_dir?;
        return 3;
    }
}
