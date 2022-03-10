
# 张汉东的 Rust 实战课
- https://time.geekbang.org/course/intro/100060601
- https://www.bilibili.com/video/BV15U4y177oh?p=2&spm_id_from=pageDriver
- https://www.youtube.com/watch?v=oiqGaAuoQW4&ab_channel=%E5%AD%A6%E4%B9%A0%E8%B5%84%E6%96%99



## 第一章：Rust语言基础 (32讲)
- 01 | 课程介绍

- 02 | 内容综述

- 03 | Rust语言学习观
  - 从整体出发， 把握一致性规则
  - 分层次递进学习  内存管理 -> 类型系统 -> 所有权， 编程范式
  - 与已有知识关联
  - 阅读源码， 第三方库
  - 主题式阅读：比如异步
  - 设计哲学

- 04 | Rust语言概览
  - 内存安全
    - 禁止对空指针，悬垂指针进行解引用
    - 禁止读取未初始化的内存
    - 缓冲区溢出
    - 禁止释放已释放或未分配的指针
  - 并发安全
  - 性能提升

- 05 | 语法面面观：词法结构
  - 编译过程

- 06 | 语法面面观：面向表达式（上）
  一切都是表达式

  fizzbuzz sample

- 07 | 语法面面观：面向表达式（中）
  CTFE(compile time funciton evaluation)
  - const fn
  - const generic

- 08 | 语法面面观：面向表达式（下）
 - 表达式分类
  

- 09 | 语法面面观：数据类型 （上）
  - 基本数据类型
  - 复合类型
  - 容器
  - 范型
  - 特定类型

- 10 | 语法面面观：数据类型 （下）
类型的行为: Trait

- 11 | 语法面面观：函数与闭包（上）
  函数
  函数指针
  闭包


- 12 | 语法面面观：函数与闭包（中）
  - 未捕捉环境变量
  - 捕捉+修改环境变量
  - 捕捉未修改环境变量

- 13 | 语法面面观：函数与闭包（下）
- 逃逸闭包
- 非逃逸闭包

- 14 | 语法面面观：模式匹配

- 15 | 语法面面观：智能指针（上）

- 16 | 语法面面观：智能指针（下）

- 17 | 语法面面观：字符与字符串（上）
  - 读文档： 类型自身， 类型方法， trait

- 18 | 语法面面观：字符与字符串（下）
  - 读文档
  
- 19 | 语法面面观：集合容器（上）

- 20 | 语法面面观：集合容器（下）


- 21 | 语法面面观：迭代器（上） 

- 22 | 语法面面观：迭代器（下）

- 23 | Rust语法面面观：模块

- 24 | Rust 语法面面观：Cargo包管理器（上）

- 25 | Rust 语法面面观：Cargo包管理器（下）

- 26 | 语法面面观：实际项目的组织结构（上）

- 27 | 语法面面观：实际项目的组织结构（下）

- 28 | 语法面面观：定义自己的Crate（上）

- 29 | 语法面面观：定义自己的Crate（中）

- 30 | 语法面面观：定义自己的Crate（下）

- 31 | 作业&第二章预告
## 第二章：Rust语言核心概念 (56讲)   
32 | 本章内容介绍：Rust语言架构
33 | 所有权：内存管理基础知识
34 | 所有权：安全管理之内存安全
35 | 所有权：Copy语义和Copy trait
36 | 所有权：深入理解Copy行为
37 | 所有权：深入理解Move语义
38 | 所有权：Move与析构
39 | 借用检查： 完全理解Scope和NLL
40 | 借用检查： 深入理解生命周期和生命周期参数
41 | 借用检查： 深入理解生命周期参数Early bound
42 | 借用检查：深入理解生命周期参数Tvs&T
43 | 借用检查： 深入理解生命周期参数： trait对象的生命周期参数
44 | 借用检查： 深入理解生命周期参数：高阶生命周期（上）
45 | 借用检查： 深入理解生命周期参数：高阶生命周期（中）
46 | 借用检查： 深入理解生命周期参数：高阶生命周期（下）
47 | 线程与并发：理解线程与并发
48 | 线程与并发：线程间安全共享数据
49 | 线程与并发：构建「无悔」并发系统（一）
50 | 线程与并发：构建「无悔」并发系统（二）
51 | 线程与并发：构建「无悔」并发系统（三）
52 | 线程与并发：构建「无悔」并发系统（四）
53 | 线程与并发：无锁并发（上）
54 | 线程与并发：无锁并发（中）
55 | 线程与并发：无锁并发（下）
56 | trait与泛型：trait静态分发
57 | trait与泛型：认识trait对象
58 | trait与泛型：泛型和trait实现模板方法
59 | trait与泛型：trait对象本质
60 | trait与泛型：对象安全本质
61 | trait与泛型：利用Enum代替trait对象
62 | trait与泛型：trait覆盖实现的一个解决方案
63 | trait与泛型：trait对象与Sized
64 | trait与泛型：trait对象与Box Self
65 | 编程范式：Rust语言编程范式讨论（上）
66 | 编程范式：Rust语言编程范式讨论（下）
67 | Rust错误处理概要
68 | Rust错误处理：Option
69 | Rust错误处理：Result（上）
70 | Rust错误处理：Result（下）
71 | Rust错误处理：try
72 | Rust错误处理：Panic
73 | Rust元编程之反射
74 | Rust元编程之反射的两种应用思路
75 | Rust元编程之编译过程与宏展开概述
76 | Rust元编程之声明宏上
77 | Rust元编程之声明宏下
78 | Rust元编程之过程宏三件套介绍
79 | Rust元编程之过程宏之Bang宏实现原理
80 | Rust元编程之过程宏-使用配置文件动态生成代码
81 | Rust元编程之过程宏Derive宏案例
82 | Rust元编程之过程宏属性宏
83 | 客观理解Unsafe Rust
84 | Unsafe Rust之安全抽象
85 | Unsafe Rust安全抽象之Drop检查
86 | Unsafe Rust安全抽象之型变
87 | UnsafeRust之其他

## 第三章：Rust异步编程基础 (52讲)
88 | Rust异步编程之IO模型
89 | Rust异步编程之epoll和io_uring
90 | Rust异步编程之事件驱动编程模型
91 | Rust异步编程之epoll代码实践
92 | Rust异步编程之Reactor代码实践
93 | Rust异步编程之MiniMio代码实践
94 | Rust异步编程之Mio代码实践(上）
95 | Rust异步编程之Mio代码实践(下）
96 | Rust异步编程之异步编程模型概要
97 | Rust异步编程之Future和Futures-rs介绍
98 | Rust异步编程之编写异步echo服务(上)
99 | Rust异步编程之编写异步echo服务(中)
100 | Rust异步编程之编写异步echo服务(下)
101 | Rust异步编程之深入理解异步Task模型
102 | Rust异步编程之Waker实现
103 | Rust异步编程之Futures库源码导读（一）
104 | Rust异步编程之Futures库源码导读（二）
105 | Rust异步编程之Futures库源码导读（三）
106 | Rust异步编程之Futures库源码导读（四）
107 | Rust异步编程之async-await语法背后
108 | Rust异步编程之生成器（上）
109 | Rust异步编程之生成器(下)
110 | Rust异步编程之Pin与Unpin(一)
111 | Rust异步编程之Pin与Unpin(二)
112 | Rust异步编程之Pin与Unpin(三）
113 | Rust异步编程之Pin与Unpin（四）
114 | Rust异步编程之no-std异步生态介绍
115 | Rust异步编程之实现异步缓存（上）
116 | Rust异步编程之实现异步缓存（下）
117 | Rust异步编程之异步运行时生态介绍
118 | Rust异步编程之smol运行时（一）
119 | Rust异步编程之smol运行时（二）
120 | Rust异步编程之smol运行时（三）
121 | Rust异步编程之smol运行时（四）
122 | Rust异步编程之smol运行时（五）
123 | Rust异步编程之smol运行时（六）
124 | Rust异步编程之smol运行时（七）
125 | Rust异步编程之smol运行时（八）
126 | Rust异步编程之smol运行时（九）
127 | Rust异步编程之smol运行时（十）
128 | Rust异步编程之async-std运行时（一）
129 | Rust异步编程之async-std运行时（二）
130 | Rust异步编程之tokio运行时（一）
131 | Rust异步编程之tokio运行时（二）
132 | Rust异步编程之tokio运行时（三）
133 | Rust异步编程之tokio运行时（四）
134 | Rust异步编程之tokio运行时（五）
135 | Rust异步编程之tokio运行时（六）
136 | Rust异步编程之tokio运行时（七）
137 | Rust异步编程之tokio运行时（八）
138 | Rust异步编程之tokio运行时（九）
139 | Rust异步编程之tokio运行时（十）
## 第四章：构建自己的异步Web框架 (34讲)
140 | Rust异步Web框架开篇
141 | Rust异步Web框架之Rocket（一）
142 | Rust异步Web框架之Rocket（二）
143 | Rust异步Web框架之Rocket（三）
144 | Rust异步Web框架之tide
145 | Rust异步Web框架之actix-web（一）
146 | Rust异步Web框架之actix-web（二）
147 | Rust异步Web框架之gotham-and-thruster
148 | Rust异步Web框架之tower（一）
149 | Rust异步Web框架之tower（二）
150 | Rust异步Web框架之hyper（一）
151 | Rust异步Web框架之hyper（二）
152 | Rust异步Web框架之hyper（三）
153 | Rust异步Web框架之warp
154 | Web框架实战之http库介绍
155 | Web框架实战之了解hyper-tower_http-http-body之间的层次关系
156 | Web框架实战之创建初始项目
157 | Web框架实战之设计框架接口
158 | Web框架实战之实现路由结构
159 | Web框架实战之实现路由结构（二）
160 | Web框架实战之实现路由结构（三）
161 | Web 框架实战之实现路由结构（四）
162 | Web 框架实战之实现路由结构（五）
163 | Web 框架实战之实现 Handler（一）
164 | Web 框架实战之实现 Handler（二）
165 | Web 框架实战之实现 Handler（三）
166 | Web 框架实战之实现 Handler（四）
167 | Web 框架实战之添加 tracing 打印
168 | Web 框架实战之实现提取器（一）
169 | Web 框架实战之实现提取器（二）
170 | Web 框架实战之实现提取器（三）
171 | Web 框架实战之实现提取器和中间件（四）
172 | Web 框架实战之错误处理支持
173 | 课程完结回顾