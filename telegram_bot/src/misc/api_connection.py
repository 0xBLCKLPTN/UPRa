import requests
from typing import List
import ast
async def recognize_phone_number(phone_number: str):
    try:
        r = requests.get(f'http://127.0.0.1:5800/phoneComment?number={phone_number}')
        if r.text != "[]":
            names_arr = ast.literal_eval(r.text)
            names_text = "\n".join(i for i in names_arr)
            return names_text
        else:
            return "К сожалению, у пользователя еще нет тегов."
    except:
        return "Пользователь не найден."
