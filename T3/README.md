## 2023_ZJU_RUST_FINAL

这里有点问题，proxy可以发送到对应的主节点和从节点上但是处理的时候没把参数传完，会panic，换句话说就是可以观察到proxy将command分发给不同的节点，但是因为一些原因没有执行好罢了（不想改了牡蛎牡蛎desu）

食用方法 ```run --bin server <proxy> <master> [slaves] 1141514 <master> [slaves] 114514 ...```

然后启动对应的 master slave 的 server 和 client 可以看到具体分发到哪个节点上了
