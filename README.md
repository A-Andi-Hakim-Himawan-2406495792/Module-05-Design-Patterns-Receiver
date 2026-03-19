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
-   [ ] Clone https://gitlab.com/ichlaffterlalu/bambangshop-receiver to a new repository.
-   **STAGE 1: Implement models and repositories**
    -   [ ] Commit: `Create Notification model struct.`
    -   [ ] Commit: `Create SubscriberRequest model struct.`
    -   [ ] Commit: `Create Notification database and Notification repository struct skeleton.`
    -   [ ] Commit: `Implement add function in Notification repository.`
    -   [ ] Commit: `Implement list_all_as_string function in Notification repository.`
    -   [ ] Write answers of your learning module's "Reflection Subscriber-1" questions in this README.
-   **STAGE 3: Implement services and controllers**
    -   [ ] Commit: `Create Notification service struct skeleton.`
    -   [ ] Commit: `Implement subscribe function in Notification service.`
    -   [ ] Commit: `Implement subscribe function in Notification controller.`
    -   [ ] Commit: `Implement unsubscribe function in Notification service.`
    -   [ ] Commit: `Implement unsubscribe function in Notification controller.`
    -   [ ] Commit: `Implement receive_notification function in Notification service.`
    -   [ ] Commit: `Implement receive function in Notification controller.`
    -   [ ] Commit: `Implement list_messages function in Notification service.`
    -   [ ] Commit: `Implement list function in Notification controller.`
    -   [ ] Write answers of your learning module's "Reflection Subscriber-2" questions in this README.

## Your Reflections
This is the place for you to write reflections:

### Mandatory (Subscriber) Reflections

#### Reflection Subscriber-1
Pertanyaan 1

Why is RwLock necessary, and why don't we use Mutex instead?

Dalam tutorial ini, saya menggunakan RwLock<Vec<Notification>> untuk mensinkronisasi akses ke list notifikasi. RwLock diperlukan karena aplikasi receiver ini berjalan dengan banyak thread secara bersamaan, ada thread yang menerima notifikasi baru (write) dan ada thread yang membaca list notifikasi untuk ditampilkan (read). Tanpa mekanisme sinkronisasi, dua thread yang mengakses Vec yang sama secara bersamaan bisa menyebabkan data race yang membuat program crash atau menghasilkan data yang korup.
Alasan saya tidak menggunakan Mutex adalah karena pola akses di aplikasi ini lebih banyak read daripada write. Mutex hanya mengizinkan satu thread mengakses data pada satu waktu, baik itu read maupun write. Artinya kalau ada 10 thread yang ingin membaca list notifikasi secara bersamaan, mereka tetap harus antri satu per satu meskipun operasi read itu sebenarnya aman dilakukan secara paralel. RwLock lebih cocok karena ia mengizinkan banyak thread membaca secara bersamaan (multiple readers), dan hanya memblokir semua akses ketika ada operasi write. Ini jauh lebih efisien untuk kasus kita di mana operasi read (menampilkan notifikasi) jauh lebih sering terjadi dibanding operasi write (menerima notifikasi baru).

Pertanyaan 2

Why doesn't Rust allow mutating static variables directly like Java?

Di Java, kita bisa dengan mudah mengubah isi static variable lewat static method karena Java menggunakan garbage collector dan memory management yang lebih longgar. Rust tidak mengizinkan hal ini karena Rust menganut prinsip ownership dan borrowing yang ketat untuk menjamin memory safety tanpa garbage collector. Static variable di Rust hidup sepanjang program berjalan dan bisa diakses dari banyak thread sekaligus, sehingga kalau Rust mengizinkan mutasi langsung tanpa mekanisme sinkronisasi, ini akan membuka celah data race yang sangat berbahaya.
Inilah mengapa kita menggunakan lazy_static, library ini memungkinkan kita mendefinisikan static variable yang diinisialisasi saat pertama kali diakses (lazy initialization), bukan saat compile time. Dengan membungkus Vec di dalam RwLock, kita memberitahu Rust bahwa kita sadar ada potensi concurrent access dan kita sudah menyediakan mekanisme sinkronisasinya. Rust kemudian percaya pada kita dan mengizinkan mutasi tersebut. Ini adalah trade-off yang disengaja oleh Rust: sedikit lebih verbose, tapi jauh lebih aman karena programmer dipaksa untuk berpikir tentang thread-safety sejak awal.

#### Reflection Subscriber-2
Pertanyaan 1

Have you explored things outside of the steps in the tutorial, for example: src/lib.rs?

Saya sempat membaca src/lib.rs untuk memahami bagaimana konfigurasi aplikasi receiver ini bekerja. Di sana saya menemukan definisi APP_CONFIG yang menggunakan lazy_static dan dotenvy untuk membaca environment variable dari file .env. Saya juga menemukan REQWEST_CLIENT yang merupakan HTTP client yang di-share across seluruh aplikasi, ini menarik karena dengan menjadikannya static, kita tidak perlu membuat client baru setiap kali ingin melakukan HTTP request, yang tentunya lebih efisien. Selain itu saya juga memahami bagaimana AppConfig menggunakan Figment untuk menggabungkan nilai default dengan environment variable, sehingga konfigurasi seperti port dan instance name bisa diubah tanpa mengubah kode.

Pertanyaan 2

How does Observer pattern ease you to plug in more subscribers? How about spawning more than one instance of Main app?

Observer pattern sangat memudahkan penambahan subscriber baru. Untuk menambah subscriber, saya cukup menjalankan instance baru dari receiver app dengan port yang berbeda, lalu melakukan HTTP POST ke endpoint /subscribe dengan URL instance tersebut. Main app tidak perlu diubah sama sekali karena ia hanya menyimpan daftar URL subscriber dan mengirim notifikasi ke semua yang terdaftar, ia tidak peduli ada berapa subscriber atau siapa mereka. Ini adalah keunggulan utama Observer pattern, yaitu publisher dan subscriber benar-benar decoupled.
Namun untuk spawning lebih dari satu instance Main app, situasinya berbeda. Karena SUBSCRIBERS disimpan sebagai in-memory static variable menggunakan DashMap, setiap instance Main app memiliki daftar subscriber-nya sendiri yang tidak dibagikan ke instance lain. Artinya kalau saya subscribe ke instance Main app pertama, instance kedua tidak akan tahu tentang subscriber tersebut dan tidak akan mengirim notifikasi. Untuk mengatasi ini, kita perlu external shared storage seperti database atau Redis agar semua instance Main app bisa berbagi daftar subscriber yang sama.

Pertanyaan 3

Have you tried to make your own Tests, or enhance documentation on your Postman collection?

Saya mencoba menambahkan deskripsi pada setiap request di Postman collection agar lebih mudah dipahami saat digunakan kembali di masa mendatang. Fitur ini sangat berguna terutama untuk Group Project karena anggota tim lain bisa langsung memahami tujuan setiap endpoint tanpa harus membaca kode. Saya juga mencoba fitur environment variable di Postman untuk menyimpan base URL main app dan receiver app, sehingga kalau port berubah saya cukup mengubah di satu tempat saja tanpa harus mengedit satu per satu di setiap request. Ke depannya saya tertarik untuk mencoba fitur automated testing di Postman yang bisa memvalidasi response secara otomatis menggunakan JavaScript.