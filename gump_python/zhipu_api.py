# 尝试使用智谱清言的API进行翻译

from zhipuai import ZhipuAI

def zhipu_api_func(apikey:str, keywords:str):
    # client = ZhipuAI(api_key="8dd2a44e60e00dbcf6fa29f64a8370f4.fnSbowuULE6ESblK")
    client = ZhipuAI(api_key=apikey)
    response = client.chat.completions.create(
        model="glm-4",  # 填写需要调用的模型名称
        messages=[
            {"role": "system", "content": "你是一个将中文翻译成英文的助手"},
            {"role": "user", "content": keywords},
        ],
    )
    print(response.choices[0].message.content)

def cmd_line():
    import argparse
    args = argparse.ArgumentParser(description='智谱清言API的ID')
    args.add_argument('-i', '--appid', type=str, help='智谱清言API的ID')
    args.add_argument('-w', '--keyword', type=str, help='需要翻译的词')
    return args.parse_args()

if __name__ == '__main__':
    args = cmd_line()
    zhipu_api_func(args.appid, args.keyword)
