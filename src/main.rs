mod views;
mod to_do;
mod state;
mod processes;
mod json_serialization;
mod jwt;

use actix_service::Service;
use actix_web::{App, HttpServer};
use actix_cors::Cors;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        let app = App::new()
            .wrap_fn(|req,srv| {
                println!("{:?}", req);
                let future = srv.call(req);
                async {
                    let result = future.await?;
                    println!("{:?}",result);
                    Ok(result)
                }
            })
            .configure(views::views_factory).wrap(cors);
        app
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}


#[cfg(test)]
mod tests {
    use std::thread;
    use std::thread::JoinHandle;
    use std::time::{Duration, Instant};


    #[test]
    fn test() {
        fn do_something(number: i8) -> i8 {
            println!("number {number} is running");
            let two_secs = Duration::new(2, 0);
            thread::sleep(two_secs);
            2
        }

        let now = Instant::now();
        let thread_one: JoinHandle<i8> = thread::spawn(|| do_something(1));
        let thread_two: JoinHandle<i8> = thread::spawn(|| do_something(2));
        let thread_three: JoinHandle<i8> = thread::spawn(|| do_something(3));

        let res_one = thread_one.join().expect("Something went wrong!");
        let res_two = thread_two.join().expect("Something went wrong!");
        let res_three = thread_three.join().expect("Something went wrong!");

        println!("time elapsed : {:?}", now.elapsed());
        println!("result: {}", res_one + res_two + res_three);
    }
}