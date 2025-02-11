mod args;
mod git;
mod git_tests;
mod run;

fn main() {
    use run::run;
    run();
}
