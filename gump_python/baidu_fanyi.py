
# 这个项目用于尝试使用网页方式调用百度翻译
# 百度翻译的接口地址为：https://fanyi.baidu.com/sug
# 接口参数为：kw=关键词
# 接口返回结果为：JSON格式
# 这个好像不需要API密钥
def main_baidu_fanyi(keyword:str):
    import requests
    url = 'https://fanyi.baidu.com/sug'
    data = {'kw': keyword}
    req = requests.post(url, data=data).json()
    print(req)

if __name__ == '__main__':
    main_baidu_fanyi("恐龙")
    # main_baidu_fanyi("hello")
    # main_baidu_fanyi("你好")
