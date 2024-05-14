fn u8 print_char(u8 char) {
    reg("al"; 1; char;);
    asm {
        intern_print_char:
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

fn u8 print_u8(u8 num) {
    reg("al"; 1; num;);
    asm {
        push rbx
        push r13
        mov r13, 1
        mov ah, 0
        mov bl, 10
        div bl
        mov bh, ah
        mov ah, 0
        div bl

        or al, 48
        cmp al, 48
        jne nstep0
        cmp r13, 1
        je next1
        nstep0:
        xor r13, r13
        call intern_print_char
        
        next1:
        mov al, ah
        or al, 48
        cmp al, 48
        jne nstep1
        cmp r13, 1
        je next2
        nstep1:
        xor r13, r13
        call intern_print_char
        
        next2:
        mov al, bh
        or al, 48
        call intern_print_char
        
        pop r13
        pop rbx
        ret
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