Fork стиллера с отправкой в telegram
Сделана сборка архива в памяти
Пофикшены ошибки
Вырезан wallet (потом добавлю)
Написано два сервера (php gate, rust server)
Отправка теперь идёт через tcp

Вес файла dev -> 15mb~
Вес файла release -> 4mb ~ (С оптимизациями)
Вес файла с release + upx -> 1.7mb +-

Стиллит
Chrome [+]
  Cookie [+]
  Password [+]
  Name browser [+]
  CC [+]
Process [+]
Cpu count [+]
Hard info [+]
Steam [+]
  SSFN [+]
  Session [+]
Telegram [+]
Sens_data [+]
Screen [+]

Использовалась ночная сборка с аргументами -> cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target x86_64-pc-windows-msvc --release
