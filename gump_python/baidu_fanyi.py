
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

# 百度翻译使用API方式,需要API密钥
# 主要是参考官方的Demo
import requests
import random
import json
from hashlib import md5

def baidu_fanyi_api(appid:str, appkey:str, keyword:str):

    from_lang = 'zh'
    to_lang =  'en'

    endpoint = 'http://api.fanyi.baidu.com'
    path = '/api/trans/vip/translate'
    url = endpoint + path

    salt = random.randint(32768, 65536)
    sign = make_md5(appid + keyword + str(salt) + appkey)

    # Build request
    headers = {'Content-Type': 'application/x-www-form-urlencoded'}
    payload = {'appid': appid, 'q': keyword, 'from': from_lang, 'to': to_lang, 'salt': salt, 'sign': sign}

    # Send request
    r = requests.post(url, params=payload, headers=headers)
    result = r.json()

    # Show response
    print(json.dumps(result, indent=4, ensure_ascii=False))


# Generate salt and sign
def make_md5(s, encoding='utf-8'):
    return md5(s.encode(encoding)).hexdigest()

def cmd_line():
    import argparse
    args = argparse.ArgumentParser(description='百度翻译API的ID和密钥')
    args.add_argument('-i', '--appid', type=str, help='百度翻译API的ID')
    args.add_argument('-k', '--appkey', type=str, help='百度翻译API的密钥')
    args.add_argument('-w', '--keyword', type=str, help='需要翻译的词')
    return args.parse_args()

if __name__ == '__main__':
    # main_baidu_fanyi("恐龙")
    # main_baidu_fanyi("hello")
    # main_baidu_fanyi("你好")

    args = cmd_line()
    baidu_fanyi_api(args.appid, args.appkey, args.keyword)
