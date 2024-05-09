fn u8 print_char(u8 char) {
    reg("al"; 1; char;);
    asm {
        push rbx
        mov rsi, QWORD[p_buf_ptr]
        mov BYTE[p_buf+rsi], al
        inc rsi
        mov QWORD[p_buf_ptr], rsi

        cmp rsi, 100
        jne fi_print_char
        mov rbx, 100
        call print_buffer
        xor rbx, rbx
        mov [p_buf_ptr], rbx
        fi_print_char:
        pop rbx
    }
}

fn u8 print($u8 str) {
    asm {
        p1:  
    }
    reg("al"; 1; $str;);
    asm {
        cmp al, 0
        je p1e
    }
    print_char($str;);
    str = str + 1;
    asm {
        jmp p1
        p1e:
    }
}

fn u8 println($u8 str) {
    print(str;);
    print_char(10;);
}