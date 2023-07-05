from dispatcher import dp, Bot
from aiogram import types
from states.states import *
from aiogram.dispatcher import FSMContext
from misc.api_connection import *

@dp.message_handler(commands=["start"], state="*")
async def on_cmd_start(message: types.Message, state: FSMContext):
    await message.answer("Привет! Меня зовут UPR-bot. Я создан для того, чтобы определять кто тебе звонил. Основываюсь я на Open-Source проекте UPR-api. Я работаю по принципу GetContact, но без рекламы и со свободным исходным кодом. Меня, как и разработчика, можно найти на Github: https://github.com/0xBLCKLPTN")



@dp.message_handler(commands=["recognize"], state="*")
async def on_cmd_recognize(message: types.Message, state: FSMContext):
    await message.answer("Хорошо. Отправь мне свой номер телефона и я попытаюсь найти для тебя какую-то информацию.")
    await Recognize.phone_number.set()


@dp.message_handler(state=Recognize.phone_number, content_types=types.ContentTypes.TEXT)
async def get_phone_number(message: types.Message, state: FSMContext):
    await state.finish()
    text = f"По номеру телефона: {message.text} найдено несколько тегов:\n\n"
    names = await recognize_phone_number(message.text)
    await message.answer(text+names)

