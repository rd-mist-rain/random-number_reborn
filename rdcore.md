需要声明的是,我旨在把这个程序写成一个"外置库"类型的东西,但和dll不同,本程序更有独立性,因为它以exe的形式存在<br>
它的工作原理是接受外部参数并通过标准输入流和标准错误流返回值,这让它可以被直接使用(方法见下文),也可以被某一个其他程序调用<br>
我们会为rdcore提供一个默认的实现,既random-number_reborn,所以rdcore的所有内容几乎都是以random-number_reborn的需要为主的
(当然也有可能rdcore会加更多的内容)
# 我该如何使用这个程序？
## 我是开发者
## 我是用户
打开cmd,先输入程序路径(如.\rdcore.exe,下文用rdcore代指程序路径)<br>
整个调用链类似这样rdcore <指令> ......(其他参数),让我在后文为你介绍一些指令吧！
# rdcore所接受的指令
## choice
choice的整个调用链类似这样rdcore choice \<starter\> \<ender\> \<amount(可选,不输入则设置为1)\><br>
这个指令的含义是:在\<starter\>(含)-\<ender\>(含)之间随机选取\<amount\>个整数并输出<br>
如:调用rdcore choice 1 50 5就代表在1-50之间随机选取5个整数并输出
