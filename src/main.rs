use axum::body::Body;
use axum::{
    routing::{get, get_service},
    Router,
    Json,
    http::HeaderMap,
    response::{Response, Html},
    http::StatusCode,
    http::Method,
    middleware::Next,
    http::Request,
};
use std::env;
use std::net::SocketAddr;
use std::collections::HashMap;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let port = env::var("PING_LISTEN_PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .expect("Le port doit être un nombre valide");

    let app = Router::new()
        .route("/", get(index_handler))
        .route("/ping", get(ping_handler))
        .fallback(handler_404)
        .layer(axum::middleware::from_fn(check_method));

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("Serveur en écoute sur {}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn ping_handler(method: Method, headers: HeaderMap) -> Result<Json<HashMap<String, String>>, Response<Body>> {
    if method != Method::GET {
        return Err(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::empty())
            .unwrap());
    }

    let headers_map: HashMap<String, String> = headers
        .iter()
        .map(|(key, value)| {
            (
                key.to_string(),
                value.to_str().unwrap_or_default().to_string(),
            )
        })
        .collect();

    Ok(Json(headers_map))
}

async fn handler_404() -> Response<Body> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::empty())
        .unwrap()
}

async fn check_method(
    request: Request<Body>,
    next: Next,
) -> Response<Body> {
    if request.uri().path() == "/ping" && request.method() != Method::GET {
        return Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::empty())
            .unwrap();
    }
    next.run(request).await
}
// testeur pour le tp api header
async fn index_handler() -> Html<&'static str> {
    Html(r#"
    <!DOCTYPE html>
    <html lang="fr">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>TP API Headers - Test</title>
        <style>
            body {
                font-family: Arial, sans-serif;
                max-width: 800px;
                margin: 0 auto;
                padding: 20px;
                background-color: #f5f5f5;
            }
            h1, h2 {
                color: #333;
                text-align: center;
            }
            .container {
                background-color: white;
                border-radius: 8px;
                padding: 20px;
                box-shadow: 0 2px 4px rgba(0,0,0,0.1);
                margin-bottom: 20px;
            }
            .test-section {
                margin-bottom: 30px;
                border-bottom: 1px solid #eee;
                padding-bottom: 20px;
            }
            .test-section:last-child {
                border-bottom: none;
            }
            button {
                background-color: #4CAF50;
                color: white;
                border: none;
                padding: 10px 15px;
                border-radius: 4px;
                cursor: pointer;
                font-size: 16px;
                margin-right: 10px;
                margin-bottom: 10px;
            }
            button:hover {
                background-color: #45a049;
            }
            button.post {
                background-color: #2196F3;
            }
            button.post:hover {
                background-color: #0b7dda;
            }
            button.other {
                background-color: #ff9800;
            }
            button.other:hover {
                background-color: #e68a00;
            }
            pre {
                background-color: #f8f8f8;
                border: 1px solid #ddd;
                border-radius: 4px;
                padding: 15px;
                overflow-x: auto;
                min-height: 50px;
            }
            .status {
                font-weight: bold;
                margin-top: 10px;
            }
            .success {
                color: #4CAF50;
            }
            .error {
                color: #f44336;
            }
            .info-box {
                background-color: #e7f3fe;
                border-left: 6px solid #2196F3;
                padding: 10px;
                margin-bottom: 15px;
            }
        </style>
    </head>
    <body>
        <h1>Testeur pour TP API Headers</h1>
        
        <div class="container">
            <div class="info-box">
                <p><strong>Port actuel:</strong> <span id="currentPort"></span></p>
                <p><strong>Note:</strong> Cette interface permet de vérifier que toutes les exigences du TP sont respectées.</p>
            </div>
        </div>

        <div class="container test-section">
            <h2>Test 1: GET /ping</h2>
            <p>Doit retourner les headers au format JSON avec code 200</p>
            <button id="testGet">Tester GET /ping</button>
            <div class="status" id="getStatus"></div>
            <pre id="getResult">Cliquez sur le bouton pour tester...</pre>
        </div>

        <div class="container test-section">
            <h2>Test 2: Autres méthodes sur /ping</h2>
            <p>Doit retourner une réponse vide avec code 404</p>
            <button class="post" id="testPost">Tester POST /ping</button>
            <button class="post" id="testPut">Tester PUT /ping</button>
            <button class="post" id="testDelete">Tester DELETE /ping</button>
            <div class="status" id="otherMethodStatus"></div>
            <pre id="otherMethodResult">Cliquez sur un bouton pour tester...</pre>
        </div>

        <div class="container test-section">
            <h2>Test 3: Autres routes</h2>
            <p>Doit retourner une réponse vide avec code 404</p>
            <button class="other" id="testOtherRoute">Tester GET /autre-route</button>
            <div class="status" id="otherRouteStatus"></div>
            <pre id="otherRouteResult">Cliquez sur le bouton pour tester...</pre>
        </div>

        <script>
            // Afficher le port actuel
            document.getElementById('currentPort').textContent = window.location.port || '80';

            // Test 1: GET /ping
            document.getElementById('testGet').addEventListener('click', async () => {
                try {
                    const response = await fetch('/ping');
                    const statusEl = document.getElementById('getStatus');
                    const resultEl = document.getElementById('getResult');
                    
                    if (response.ok) {
                        const data = await response.json();
                        statusEl.textContent = `Statut: ${response.status} ${response.statusText} ✅`;
                        statusEl.className = 'status success';
                        resultEl.textContent = JSON.stringify(data, null, 2);
                    } else {
                        statusEl.textContent = `Statut: ${response.status} ${response.statusText} ❌`;
                        statusEl.className = 'status error';
                        resultEl.textContent = 'Erreur: La requête GET /ping n\'a pas retourné un statut 200';
                    }
                } catch (error) {
                    document.getElementById('getStatus').textContent = 'Erreur ❌';
                    document.getElementById('getStatus').className = 'status error';
                    document.getElementById('getResult').textContent = 'Erreur: ' + error.message;
                }
            });

            // Test 2: Autres méthodes sur /ping
            async function testMethod(method) {
                try {
                    const response = await fetch('/ping', { method });
                    const statusEl = document.getElementById('otherMethodStatus');
                    const resultEl = document.getElementById('otherMethodResult');
                    
                    if (response.status === 404) {
                        statusEl.textContent = `Statut: ${response.status} ${response.statusText} ✅`;
                        statusEl.className = 'status success';
                        
                        // Afficher les headers de la réponse
                        const headers = {};
                        response.headers.forEach((value, key) => {
                            headers[key] = value;
                        });
                        
                        resultEl.textContent = `La méthode ${method} sur /ping retourne bien 404 comme demandé\n\nHeaders de la réponse:\n${JSON.stringify(headers, null, 2)}`;
                    } else {
                        statusEl.textContent = `Statut: ${response.status} ${response.statusText} ❌`;
                        statusEl.className = 'status error';
                        resultEl.textContent = `Erreur: La méthode ${method} sur /ping devrait retourner 404 mais a retourné ${response.status}`;
                    }
                } catch (error) {
                    document.getElementById('otherMethodStatus').textContent = 'Erreur ❌';
                    document.getElementById('otherMethodStatus').className = 'status error';
                    document.getElementById('otherMethodResult').textContent = 'Erreur: ' + error.message;
                }
            }

            document.getElementById('testPost').addEventListener('click', () => testMethod('POST'));
            document.getElementById('testPut').addEventListener('click', () => testMethod('PUT'));
            document.getElementById('testDelete').addEventListener('click', () => testMethod('DELETE'));

            // Test 3: Autres routes
            document.getElementById('testOtherRoute').addEventListener('click', async () => {
                try {
                    const response = await fetch('/autre-route');
                    const statusEl = document.getElementById('otherRouteStatus');
                    const resultEl = document.getElementById('otherRouteResult');
                    
                    if (response.status === 404) {
                        statusEl.textContent = `Statut: ${response.status} ${response.statusText} ✅`;
                        statusEl.className = 'status success';
                        
                        // Afficher les headers de la réponse
                        const headers = {};
                        response.headers.forEach((value, key) => {
                            headers[key] = value;
                        });
                        
                        resultEl.textContent = `La route /autre-route retourne bien 404 comme demandé\n\nHeaders de la réponse:\n${JSON.stringify(headers, null, 2)}`;
                    } else {
                        statusEl.textContent = `Statut: ${response.status} ${response.statusText} ❌`;
                        statusEl.className = 'status error';
                        resultEl.textContent = `Erreur: La route /autre-route devrait retourner 404 mais a retourné ${response.status}`;
                    }
                } catch (error) {
                    document.getElementById('otherRouteStatus').textContent = 'Erreur ❌';
                    document.getElementById('otherRouteStatus').className = 'status error';
                    document.getElementById('otherRouteResult').textContent = 'Erreur: ' + error.message;
                }
            });
        </script>
    </body>
    </html>
    "#)
}
