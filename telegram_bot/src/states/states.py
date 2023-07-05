from aiogram.dispatcher.filters.state import State, StatesGroup


class Recognize(StatesGroup):
    phone_number = State()

