mod git;

use git2::{Time};
use crate::git::{Commit, Git, Repo};

fn print_commit(commit: &Commit) {
    println!("commit {}", &commit.id);

    // if commit.parents().len() > 1 {
    //     print!("Merge:");
    //     for id in commit.parent_ids() {
    //         print!(" {:.8}", id);
    //     }
    //     println!();
    // }

    let author = &commit.author;
    println!("Author: {}", author);
    print_time(&commit.time, "Date:   ");
    println!();

    for line in commit.message.lines() {
        println!("    {}", line);
    }
    println!();
}

fn print_time(time: &Time, prefix: &str) {
    let (offset, sign) = match time.offset_minutes() {
        n if n < 0 => (-n, '-'),
        n => (n, '+'),
    };
    let (hours, minutes) = (offset / 60, offset % 60);
    let ts = time::Timespec::new(time.seconds() + (time.offset_minutes() as i64) * 60, 0);
    let time = time::at(ts);

    println!(
        "{}{} {}{:02}{:02}",
        prefix,
        time.strftime("%a %b %e %T %Y").unwrap(),
        sign,
        hours,
        minutes
    );
}

fn run() {
    let path = "../../vcanr";
    let repo = Git {path: String::from(path)};
    let commits = repo.commits();
    for commit in &commits {
        print_commit(commit);
    }
}

fn main() {
    run();
}