use anyhow::private::new_adhoc;
use text_over_image::{image_generator::Ticket, mail_sender::MailSender};
use std::path;

fn main() {

    // esto es un ejemplo indecente de cómo se usa

    let subject = "Tiquete de abordaje Semana de la Astronomía 2021";
    let image = path::Path::new("/home/fabian/bin/text_over_image/SA2021_ticket.png");

    let smtp_server = "smtp.gmail.com";
    let body = "¡Hola! Saludos de parte del equipo de AstroTEC :)\n\nRecibimos tu formulario para adquirir el tiquete de abordaje de la Semana de la Astronomía 2021 con tu nombre, y aquí te lo traemos. Podés compartirlo en tus redes sociales y etiquetarnos, así también nos ayudás a difundir la voz sobre el evento.\n\n¡Te esperamos la semana del 15 al 21 de marzo!";
    let username = "contacto.astrotec@gmail.com";
    let password = "";
    let to = "";

    let nombre = "Test";

    let ticket = Ticket::new(image, nombre);
    let mailer = MailSender::new(&smtp_server, &username, &password);
    
    mailer.send_astrotec_ticket(to, subject, body, ticket.print("SA2021_ticket_filled.png").unwrap())

}
