pub struct MailTemplate;
impl MailTemplate 
{
    pub fn forgot_password_template(username: String, path: String) -> String
    {
        let css = "<style>
        .message-content
        {
          font-family: Arial, Helvetica, sans-serif;
          max-width: 600px;
          font-size: 18px;
          line-height: 24px;
        }
        </style>";

        let body_html = format!(
            r#"{}
            <div class="message-content">
                <p>Olá {}!</p>
                <br>
                <p>Recebemos uma solicitação de redefinição de senha para sua conta de usuário.</p>
                <p>Se realmente foi você que solicitou, clique no link abaixo para escolher uma nova senha:</p>
                <p>
                <a href="{}">Resetar minha senha</a>
                </p>
                <p>Caso você não tenha realizado esta solicitação, ignore este email.</p>
                <br>
                <p>Obrigado!</p>
                <p>Equipe API Vendas.</p>
            </div>"#, css, username, path
        );

        body_html
    }    
}