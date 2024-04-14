# BambangShop Receiver App
Tutorial and Example for Advanced Programming 2024 - Faculty of Computer Science, Universitas Indonesia

---

## About this Project
In this repository, we have provided you a REST (REpresentational State Transfer) API project using Rocket web framework.

This project consists of four modules:
1.  `controller`: this module contains handler functions used to receive request and send responses.
    In Model-View-Controller (MVC) pattern, this is the Controller part.
2.  `model`: this module contains structs that serve as data containers.
    In MVC pattern, this is the Model part.
3.  `service`: this module contains structs with business logic methods.
    In MVC pattern, this is also the Model part.
4.  `repository`: this module contains structs that serve as databases.
    You can use methods of the struct to get list of objects, or operating an object (create, read, update, delete).

This repository provides a Rocket web framework skeleton that you can work with.

As this is an Observer Design Pattern tutorial repository, you need to implement a feature: `Notification`.
This feature will receive notifications of creation, promotion, and deletion of a product, when this receiver instance is subscribed to a certain product type.
The notification will be sent using HTTP POST request, so you need to make the receiver endpoint in this project.

## API Documentations

You can download the Postman Collection JSON here: https://ristek.link/AdvProgWeek7Postman

After you download the Postman Collection, you can try the endpoints inside "BambangShop Receiver" folder.

Postman is an installable client that you can use to test web endpoints using HTTP request.
You can also make automated functional testing scripts for REST API projects using this client.
You can install Postman via this website: https://www.postman.com/downloads/

## How to Run in Development Environment
1.  Set up environment variables first by creating `.env` file.
    Here is the example of `.env` file:
    ```bash
    ROCKET_PORT=8001
    APP_INSTANCE_ROOT_URL=http://localhost:${ROCKET_PORT}
    APP_PUBLISHER_ROOT_URL=http://localhost:8000
    APP_INSTANCE_NAME=Safira Sudrajat
    ```
    Here are the details of each environment variable:
    | variable                | type   | description                                                     |
    |-------------------------|--------|-----------------------------------------------------------------|
    | ROCKET_PORT             | string | Port number that will be listened by this receiver instance.    |
    | APP_INSTANCE_ROOT_URL   | string | URL address where this receiver instance can be accessed.       |
    | APP_PUUBLISHER_ROOT_URL | string | URL address where the publisher instance can be accessed.       |
    | APP_INSTANCE_NAME       | string | Name of this receiver instance, will be shown on notifications. |
2.  Use `cargo run` to run this app.
    (You might want to use `cargo check` if you only need to verify your work without running the app.)
3.  To simulate multiple instances of BambangShop Receiver (as the tutorial mandates you to do so),
    you can open new terminal, then edit `ROCKET_PORT` in `.env` file, then execute another `cargo run`.

    For example, if you want to run 3 (three) instances of BambangShop Receiver at port `8001`, `8002`, and `8003`, you can do these steps:
    -   Edit `ROCKET_PORT` in `.env` to `8001`, then execute `cargo run`.
    -   Open new terminal, edit `ROCKET_PORT` in `.env` to `8002`, then execute `cargo run`.
    -   Open another new terminal, edit `ROCKET_PORT` in `.env` to `8003`, then execute `cargo run`.

## Mandatory Checklists (Subscriber)
-   [x] Clone https://gitlab.com/ichlaffterlalu/bambangshop-receiver to a new repository.
-   **STAGE 1: Implement models and repositories**
    -   [x] Commit: `Create Notification model struct.`
    -   [x] Commit: `Create SubscriberRequest model struct.`
    -   [x] Commit: `Create Notification database and Notification repository struct skeleton.`
    -   [x] Commit: `Implement add function in Notification repository.`
    -   [x] Commit: `Implement list_all_as_string function in Notification repository.`
    -   [x] Write answers of your learning module's "Reflection Subscriber-1" questions in this README.
-   **STAGE 3: Implement services and controllers**
    -   [x] Commit: `Create Notification service struct skeleton.`
    -   [x] Commit: `Implement subscribe function in Notification service.`
    -   [x] Commit: `Implement subscribe function in Notification controller.`
    -   [x] Commit: `Implement unsubscribe function in Notification service.`
    -   [x] Commit: `Implement unsubscribe function in Notification controller.`
    -   [x] Commit: `Implement receive_notification function in Notification service.`
    -   [x] Commit: `Implement receive function in Notification controller.`
    -   [x] Commit: `Implement list_messages function in Notification service.`
    -   [x] Commit: `Implement list function in Notification controller.`
    -   [x] Write answers of your learning module's "Reflection Subscriber-2" questions in this README.

## Your Reflections
This is the place for you to write reflections:

### Mandatory (Subscriber) Reflections

#### Reflection Subscriber-1
1. Dalam tutorial ini, penggunaan RwLock<> untuk mengsinkronkan penggunaan Vec dari Notifikasi diperlukan karena kita ingin memungkinkan banyak pembaca untuk mengakses struktur data tersebut secara bersamaan, sementara hanya satu penulis yang dapat mengubahnya pada satu waktu. Dengan menggunakan RwLock<>, kita dapat memastikan bahwa pembaca dapat mengakses data Notifikasi secara paralel tanpa saling mengganggu, sementara penulis memiliki eksklusifitas untuk mengubah data tanpa interferensi dari pembaca. Hal ini sangat penting dalam konteks di mana kita memiliki banyak thread yang berpotensi untuk membaca data Notifikasi secara bersamaan.
Namun, kita tidak menggunakan Mutex<> dalam kasus ini karena Mutex<> hanya mengizinkan satu thread untuk memiliki akses ke data pada satu waktu baik untuk membaca maupun menulis. Dalam kasus ini, karena kita ingin memungkinkan banyak pembaca untuk mengakses data Notifikasi secara bersamaan, penggunaan Mutex<> akan menyebabkan pembaca harus menunggu hingga penulis selesai menggunakan data tersebut, bahkan jika pembaca hanya melakukan operasi baca. Ini akan mengurangi efisiensi sistem, terutama jika terjadi banyak operasi baca. Dengan RwLock<>, pembaca dapat bekerja secara paralel kecuali ketika penulis sedang mengubah data, yang dapat meningkatkan kinerja aplikasi dalam situasi di mana pembacaan data jauh lebih umum daripada penulisan.
2. Dalam tutorial ini, kita menggunakan library eksternal lazy_static untuk mendefinisikan Vec dan DashMap sebagai variabel "static". Berbeda dengan Java di mana kita dapat memutasi konten dari variabel statik melalui fungsi statik, Rust tidak mengizinkan kita untuk melakukannya karena keamanan dan pengelolaan memori yang lebih ketat. Rust menerapkan aturan peminjaman yang ketat untuk mencegah datarace dan konflik peminjaman pada waktu kompilasi. Dengan demikian, memutasi variabel statik dari fungsi statik dapat membuka kemungkinan adanya peminjaman yang tidak aman atau konflik akses yang dapat menyebabkan kesalahan di waktu kompilasi atau bahkan di waktu runtime. Oleh karena itu, Rust mendorong pendekatan yang lebih aman dengan mewajibkan penanganan peminjaman secara eksplisit dan menjamin keselamatan memori pada tingkat kompilasi.
#### Reflection Subscriber-2
1. Dalam file `src/lib.rs`, kita melihat penggunaan beberapa library eksternal dan definisi beberapa struktur data penting. Pertama, kita menggunakan `lazy_static` untuk membuat variabel global `REQWEST_CLIENT` yang merupakan instance dari `reqwest::Client`. Ini memungkinkan kita untuk memiliki satu instance `reqwest::Client` yang dapat digunakan secara global dalam aplikasi, menghindari biaya overhead pembuatan instance yang berulang-ulang. Selain itu, kita juga mendefinisikan `APP_CONFIG` sebagai variabel global yang berisi konfigurasi aplikasi seperti URL akar instance dan URL akar penerbit. Ini memungkinkan kita untuk mengakses konfigurasi aplikasi dari mana saja dalam kode tanpa perlu memperolehnya berulang kali.
Kedua, dalam `AppConfig`, kita mengimplementasikan sebuah metode `generate()` yang memuat konfigurasi aplikasi dari berkas `.env` menggunakan library `dotenv` dan kemudian menggabungkannya dengan variabel lingkungan untuk konfigurasi yang lebih spesifik. Hal ini memungkinkan kita untuk memiliki konfigurasi aplikasi yang fleksibel dan dapat dikonfigurasi dengan mudah melalui berkas `.env`. Selain itu, kita juga mendefinisikan struktur `ErrorResponse` yang digunakan untuk merangkum pesan kesalahan dan kode status HTTP yang terkait. Ini membantu dalam menangani kesalahan dan memberikan tanggapan yang tepat dalam aplikasi.
2. Dengan menerapkan pola Observer, menambahkan lebih banyak subscriber menjadi lebih mudah karena setiap subscriber hanya perlu mendaftar ke subjek atau penerbit yang tepat. Dalam kasus ini, setiap instance Receiver dapat mendaftar sebagai subscriber dengan mudah dan menerima notifikasi sesuai kebutuhan mereka. Pola Observer memisahkan logika penerbitan notifikasi dari logika subscriber, sehingga memungkinkan penambahan dan penghapusan subscriber tanpa mengubah struktur dasar dari sistem. Hal ini membuat sistem lebih modular dan memungkinkan untuk penyesuaian yang mudah tanpa harus memodifikasi banyak bagian dari kode.
Namun, ketika menduplikasi lebih dari satu instance aplikasi utama, menambahkan ke sistem mungkin tidak semudah menambahkan subscriber baru. Karena setiap instance aplikasi utama bertindak sebagai penerbit notifikasi, menambahkan lebih banyak instance akan memperkenalkan kompleksitas dalam manajemen notifikasi. Namun, dengan desain yang baik dan pemisahan konsen yang tepat antara logika penerbitan dan logika subscriber, masih mungkin untuk menangani penambahan instance aplikasi utama dengan memperhatikan koordinasi yang baik antara mereka dalam pengiriman notifikasi.
3. Saya telah mencoba membuat pengujian sendiri dan meningkatkan dokumentasi pada koleksi Postman saya. Kedua fitur tersebut membuktikan sangat berguna untuk pekerjaan saya, baik itu dalam proyek tutorial maupun proyek kelompok. Dengan membuat pengujian sendiri, saya dapat menguji fungsionalitas sistem secara lebih menyeluruh dan memastikan bahwa setiap bagian dari kode berperilaku seperti yang diharapkan. Ini membantu saya mengidentifikasi masalah potensial dan memperbaikinya sebelum mereka mencapai tahap produksi, yang pada akhirnya meningkatkan kualitas keseluruhan dari proyek yang saya kerjakan.
Sementara itu, meningkatkan dokumentasi pada koleksi Postman memungkinkan saya dan anggota tim untuk lebih mudah memahami dan menggunakan API yang telah dibangun. Ini membantu dalam komunikasi antara tim dan memastikan bahwa setiap anggota dapat dengan mudah mengakses informasi yang mereka butuhkan untuk bekerja dengan API. Dengan dokumentasi yang jelas dan lengkap, kita dapat menghindari kebingungan dan kesalahpahaman, serta meningkatkan produktivitas dalam pengembangan dan pengujian aplikasi. Dengan demikian, fitur-fitur ini membantu meningkatkan efisiensi dan kualitas pekerjaan saya baik dalam konteks tutorial maupun proyek kelompok.