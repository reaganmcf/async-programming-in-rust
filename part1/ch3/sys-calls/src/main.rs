mod normal_syscall;
mod raw_syscall;

fn main() {
    raw_syscall::syscall("Hello, world from a raw syscall!\n\n".to_string());

    normal_syscall::syscall("Hello, world from a normal syscall!".to_string())
        .expect("Failed to print out");
}
