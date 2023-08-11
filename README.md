# subx-address
生成subsrate的各种网络地址
## 使用方法
拉取项目到本地
```bash
git clone https://github.com/antaintan/subx-address.git
```
进入项目路径
```bash
cd subx-address
```

获取并更新依赖
```bash
cargo update 
```

运行main.js并启动服务端
```bash
cargo run
```

使用POST请求获取新创建的地址，以postman为例：
创建一个对``` http://localhost:8000/address/new ``` 的``` POST ```请求

便可以得到新创建的地址
```json
{
    "code": 0,
    "data": {
        "addresses": {
            "acaAddress": "21kRnSsXm2xGGKXQGc8PqWxq4wRLKiXdYJA5fsz3sAvHZbeo",
            "bncAddress": "cbnkUhvDsTMe5uYeUcJKuu8UzrA6z1YnXwZD9Qr1kYLSL1m",
            "cringAddress": "2p3hrPCkqMx5Wnm7WsAarj6EEpZgmBZo3pb8bM8Ta1gBpEck",
            "dotAddress": "1sqCXTVv8CzhjUmDHLA4WJac5zPmtERetcmX3Z3EcyQkJKz",
            "edgAddress": "iPT1UFKcAL5o9K1Z1VKhpAZ8UzTAAe4yUVgpq49Uc8piSsS",
            "fisAddress": "31d2NNHZbwhXpua3KvvdcXd5XnrGsYpqRhhPpiR4VisANj5w",
            "klpAddress": "2cUPLQ7x4nCdCfxBhoQY6vZNwrH6epJkfwUsMyqreJVDFTyN",
            "ksmAddress": "DT9iWYJghxT1rHh2M6CpJqRu4GytFVU2mj2kQqeALAPJqSS",
            "plmAddress": "Wp8VVAWqaadV2W5jwjGx1dhqWhs3oP2bbPRbTmYYtwrA5Q6"
        },
        "phrase": "forest bag bleak group million equal problem pen surge crowd approve name",
        "public": "0x26c485835b9f8fd5e482beffb53daf64ee2de0bf0dc8aa70407f0a653e923d23",
        "seed": "0xdbf75031e6a30ef37473e50a7b645ae4680e7586704698a682268a6250d8c194",
        "ss58Address": "5CwY4CCS4LwXGCUFFeH9vMURkTzk5agHaPtHMkZggXwtZxQv"
    },
    "msg": "success"
}
```
