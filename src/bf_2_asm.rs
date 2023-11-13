use std::fs;
use std::io::Write;

fn operation(char: &str, mut indent: usize, mut last_label: i32, labels: &mut Vec<i32>, num: usize) -> (String, usize, i32) {
    let indent_text: &str = &"    ".repeat(indent);
    (match char {
        "+" => {format!("{}mov r9, {}\n{}add [array+rcx], r9\n", indent_text, num, indent_text)},
        "-" => {format!("{}mov r9, {}\n{}sub [array+rcx], r9\n", indent_text, num, indent_text)},
        "<" => {format!("{}sub rcx, {}\n",indent_text, num)},
        ">" => {format!("{}add rcx, {}\n",indent_text, num)},
        "." => {format!("{}mov rax, [array+rcx]\n{}call print_char\n",indent_text, indent_text)},
        "," => {format!("{}call print_buffer\n{}call get_char\n", indent_text, indent_text)},
        "[" => {
            indent += 1;
            last_label += 1;
            labels.push(last_label);
            format!("{}label{}:\n    {}cmp [array+rcx], r10\n    {}je label{}_2\n",indent_text, last_label, indent_text, indent_text, last_label)
            
        },
        "]" => {
            indent -= 1;
            format!("{}label{}_2:\n{}cmp [array+rcx], r10\n{}jne label{}\n",indent_text, last_label, indent_text, indent_text, labels.pop().unwrap())
        },
        _ => {format!("")}
    }, indent, last_label)
}

fn load(path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(path)
}

pub fn run(input_path: &str, output_path: &str) -> u8 {
    let mut file = fs::File::create(output_path).unwrap();
    file.write_all(b"global _start
    
section .data
    array: times 100000 db 0
    buffer: times 100 db 0
    temp dq 0
    
section .text
    
print_buffer:
    push rcx

    mov rax, 1          ; syscall for syswrite
    mov rdi, 1          ; stdout file descriptor
    mov rsi, buffer     ; bytes to write (by reference?)
    mov rdx, rbx        ; number of bytes to write
    syscall             ; call syscall

    mov rbx, 0
    pop rcx
    ret

print_char:
    mov [buffer+rbx], rax
    add rbx, 1
    cmp rbx, 100
    jne no_print
    call print_buffer
    no_print:
    ret
    
get_char:
    push rcx
    push rbx
    push rax

    mov rax, 0      ; syscall for sysread
    mov rdi, 0      ; stdin file descriptor
    mov rsi, rsp    ; bytes to write (by reference?)
    mov rdx, 1      ; number of bytes to write
    syscall         ; call syscall

    mov rax, 0      ; syscall for sysread
    mov rdi, 0      ; stdin file descriptor
    mov rsi, temp   ; bytes to write (by reference?)
    mov rdx, 1      ; number of bytes to write
    syscall         ; call syscall

    pop rax
    pop rbx
    pop rcx
    mov [array+rcx], rax
    ret
    
_start:
    xor rcx, rcx    ; rcx is mem_pointer
    mov r9, 1       ; r9 is just to make memory increments
    mov r10, 0      ; r10 is just to make memory comparisons
    mov rbx, 0
    
").unwrap();
    
    let binding = match load(&input_path) {
        Ok(output) => output,
        Err(_) => return 1
    };
    let contents:&[u8] = binding.as_bytes();
    let mut buffer = String::new();
    let mut temp:String;
    let mut indent: usize = 1;
    let mut last_label: i32 = 0;
    let mut labels: Vec<i32> = vec!();

    let mut i:usize = 0;
    while i < contents.len() {
        let chr:u8 = contents[i];
        let mut k:usize = 1;
        while chr == *contents.get(i+k).unwrap_or(&0) && (chr == 43 || chr == 45 || chr == 60 || chr == 62) {k += 1}
        (temp, indent, last_label) = operation(std::str::from_utf8(&[contents[i]]).unwrap(), indent, last_label, &mut labels, k);
        i += k;
        buffer += &temp
    }

    file.write_all(buffer.as_bytes()).unwrap();

    file.write_all(b"
    call print_buffer
    mov rax, 60
    mov rdi, 0
    syscall
    ").unwrap();
    
    1
}