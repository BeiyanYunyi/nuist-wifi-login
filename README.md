# 南信大 WiFi 登录

## 用法

```powershell
.\nuist-login.exe <运营商> -p <密码> -u <用户名>
```

其中运营商值分别为
| 运营商 | 值 |
| ---------- | ------- |
| 中国移动 | cmcc |
| 中国联通 | china-unicom |
| 中国电信 | china-net |
| 南信带土著 | school |

可以添加一个快捷方式到桌面，然后在快捷方式的目标中添加参数。如果登录失败，本程序会每隔 5 秒自动重试。
