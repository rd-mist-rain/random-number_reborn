**1.0.0版本中,我大幅优化了程序项目的代码,所以1.0.0版本开始的rdcore被我们称为"nextgen"系列,但程序依然叫rdcore**<br>
需要声明的是,我旨在把这个程序写成一个"外置库"类型的东西,但和dll不同,本程序更有独立性,因为它以exe的形式存在<br>
它的工作原理是接受外部参数并通过标准输入流和标准错误流返回值,这让它可以被直接使用(方法见下文),也可以被某一个其他程序调用<br>
我们会为rdcore提供一个默认的实现,既random-number_reborn,所以rdcore的所有内容几乎都是以random-number_reborn的需要为主的
(当然也有可能rdcore会加更多的内容)
# 我该如何使用这个程序？
## 我是开发者
## 我是用户
打开cmd,先输入程序路径(如.\rdcore.exe,下文用rdcore代指程序路径)<br>
整个调用链类似这样rdcore <指令> ......(其他参数),让我在后文为你介绍一些指令吧！
# 原生rdcore所接受的指令
## choice
choice的调用链类似这样**rdcore choice \<low\> \<high\> \<amount(可选,不输入则设置为1)\>\<step\>(可选,不输入则设置为1>**<br>
这个指令的含义是:在\<low\>(含)-\<high\>(含)之间随机选取\<amount\>个整数并输出,且保证每一个输出结果满足<br>(n-low)%step=0(n-low能够整除step)<br>
如:调用rdcore choice 1 50 5就代表在1-50之间随机选取5个整数并输出<br>
·此外,你还可以在所有参数之后附加一个--sort来让程序将输出的结果按照递增来排序<br>
## choicef
choicef是choice的浮点数版本,它的调用链类似这样**rdcore choicef \<precision\> \<low\> \<high\> \<amount(可选)\>** <br>
要介绍的是精度参数precision,它代表生成随机数时会精确到小数点后几位,比如传递的precision为2,就代表生成的浮点数会精确到小数点后2位<br>

**警告:不要让 high×10^precision超过i64类型的最大值！否则会引发未定义行为** <br>
·此外,你还可以在所有参数之后附加一个--sort来让程序将输出的结果按照递增来排序<br>
**注意:使用时不要忘记第一项是precision以及不要输入步长参数,这和choice是不同的!**
# rdcore的扩展功能
rdcore在1.2.0版本新增了"扩展功能",这个功能大致是这样的:如果你输入了不是原生rdcore所接受的指令,<br>
程序就会自动寻找.\\extensions\\<输入的第一个参数名>.dll并调用dll中名称为你输入的第一个参数名(即与dll文件同名)的函数,传递的参数是一个迭代器
(在代码中你可以看到,传递的参数实际上是已经消费过第一项(程序名)的args迭代器,这是所有其余文件中定义的参数名叫"arg"而非"args"的原因) <br>
这种模式让你可以为我们的程序编写扩展功能(但只能使用Rust语言,后续可能会更改)! <br>
我自己为程序提供了一些扩展功能的dll,你可以在我们的项目的extensions目录中找到这些dll和它们的源码 <br>
**注意:下面所有扩展指令都需要你在rdcore.exe同级目录新建extensions文件夹并把与你要使用的指令同名的dll放进去才可以使用!**
## mix
mix的调用链类似这样**rdcore mix ......(你要打乱的东西,数量任意)** <br>
如:调用rdcore mix 1 2 3 4 5就代表打乱1 2 3 4 5,最后程序有可能输出2 3 1 4 5或其他,这取决于打乱的结果<br>
备注:Rust的rand crate已经有shuffle方法,而这个扩展也是用shuffle方法实现的,这个调用非常简单,所以如果你是Rust的开发者,你可以不必启用这个扩展
## choicestr
choicestr的调用链类似这样**rdcore choicestr \<amount\> ......(你要选取的东西,数量任意,但正常人的思路是至少有amount个)** <br>
和choice和choicef不同的是,这里的amount必须输入<br>
如:调用rdcore choicestr 2 str1 str2 str3 最后的结果有可能输出str1 str2或其他,这取决于选取的结果<br>
备注:Rust的rand crate已经有shuffle方法,而这个扩展也是用shuffle方法实现的,这个调用非常简单,所以如果你是Rust的开发者,你可以不必启用这个扩展
## choiceship
我感觉choiceship并不是很常用,所以把它放在了扩展中(但其实他应该是这些扩展中最常用的一个了吧?)<br>
choiceship的调用链类似这样**rdcore choiceship \<low\> \<high\> \<amount\> \<step\> --sort/--unsort --dedup/--dup \<value1\>-\<ship1\> \<value2\>-\<ship2\>...** <br>
和其他函数不同的是,这里的除了加权以外的所有参数都必须输入(因为有着数量不一定的权参数,我们不得不这样要求)<br>
--sort/--unsort --dedup/--dup中间的/表示你可以在这两项之间选一个输入,当然代码只判断是否是--sort及是否是--dedup,<br>
所以如果你不想开启排序功能或不想开启去重功能的话,随便输入些什么占位也是可以的,<br>
只不过为了可读性考虑我才选择在说明文档中使用--unsort和--dup占位<br>
**注意:使用时千万不要把平常易忽略的可选参数落下了!**

去重参数(--dedup/--dup) 注:dedup即deduplicate(去重)的缩写,dup即duplicate(保留重复)的缩写,<br>
如果你开启了去重(即输入的参数是--dedup),生成的随机数将不会有重复<br>
*choice和choicef默认是去重的,只有choiceship提供了可选的去重参数*<br>

加权参数,即\<value1\>-\<ship1\> \<value2\>-\<ship2\>...部分<br>
choiceship的实现是根据前面所有的参数生成一个Vec,然后根据加权参数将\<ship\>个\<value\>加入Vec(如果value原本就存在的话)<br>
同时,当ship为0时我们做了特殊实现(它本应该是什么也不做),即去除掉value(如果value原本就存在的话)<br>
