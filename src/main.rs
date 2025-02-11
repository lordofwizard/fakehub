mod args;
mod git;
mod git_tests;
mod run;
mod time;

fn main() {
    use run::run;
    run();
}
