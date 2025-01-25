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
