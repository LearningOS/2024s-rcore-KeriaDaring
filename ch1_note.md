# 实验步骤描述

1. 首先安装了 **qemu7.0**
使用如下命令查看qemu的相关信息
> qemu-riscv64 --version
> qemu-system-riscv64 --version

2. 在这之后将rust的 **编译平台** 从默认的aarch转移到 riscv 上
> #可以查看当前的编译目标三元组
> **rustc --version --verbose 
> 设置目标平台操作步骤如下
> 在 ${pwd}/.cargo/config 下设置build目标平台
>> [build]
>> targer="riscv64gc-unknown-none-elf"
> 之后遇到了点问题需要做额外的配置操作
>
> #添加编译目标，这样才可以让cargo正确build代码
> rustup target add riscv64gc-unknown-none-elf 
>
> 通过四个操作解除rust代码对std库的依赖
>
> 1. #![no_std]
> 2. 建立 lang_items 提供 panic 语义相关操作
>> 引入 core::panic::PanicInfo;
>> #[panic_handler] 用来对接lang_items
>> 传入 一个 panicinfo 等待后续使用
> 3. #![no_main]
> 4. 删除main函数
> 
至此可以正常编译了
3. 在这之后使用相关工具查看编译出来的二进制文件相关信息
用到了几个工具
> file #文件类型信息 哪个平台的 连接方式
> rust-readobj -h  #文件头信息 程序start地址
> rust-objdump -S  #反汇编导出汇编程序


> qemu-system-riscv64 -machine virt -nographic -bios ../bootloader/rustsbi-qemu.bin -device loader,file=target/riscv64gc-unknown-none-elf/release/os.bin
