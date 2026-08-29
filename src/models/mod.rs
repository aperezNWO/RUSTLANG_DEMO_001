#![allow(dead_code)]

use tiberius::{Client, Config, AuthMethod};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncReadCompatExt;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessLog {
    pub id_column: i32,
    pub page_name: Option<String>,
    pub access_date: Option<String>,
    pub ip_value: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct FractalParams {
    pub kind: u8,
    pub max_iterations: Option<u32>,
    pub x_min: Option<f64>,
    pub x_max: Option<f64>,
    pub y_min: Option<f64>,
    pub y_max: Option<f64>,
}

#[derive(Debug, Clone, Copy)]
pub struct Bounds {
    pub x_min: f64,
    pub x_max: f64,
    pub y_min: f64,
    pub y_max: f64,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct FractalPoint {
    pub x: f64,
    pub y: f64,
    pub intensity: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PersonaTable {
    pub id_column: i32,
    pub ciudad: Option<String>,
    pub nombre_completo: Option<String>,
}

/// Helper to create an async connection to MS SQL Server
async fn create_db_client() -> Result<Client<tokio_util::compat::Compat<TcpStream>>, Box<dyn std::error::Error + Send + Sync>> {
    let mut config = Config::new();
    config.host("webapiangulardemo.mssql.somee.com");
    config.port(1433);
    config.database("webapiangulardemo");
    config.authentication(AuthMethod::sql_server("aperezNWO_SQLLogin_1", "aperezNWO_SQLLogin_1"));
    config.trust_cert(); // Mirrors encrypt=false behavior in JDBC

    let tcp = TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;
    
    let client = Client::connect(config, tcp.compat()).await?;
    Ok(client)
}

pub async fn fetch_access_logs() -> Result<Vec<AccessLog>, Box<dyn std::error::Error + Send + Sync>> {
    let mut client = create_db_client().await?;

    let sql = "
        SELECT TOP 100
               AL.[ID_column]     AS id_column
             , AL.[PageName]      AS page_name
             , AL.[AccessDate]    AS access_date
             , AL.[IpValue]       AS ip_value
        FROM
            dbo.accessLogs AL
        WHERE
            AL.[LogType] = 1
        AND
            (AL.PAGENAME LIKE '%DEMO%'
        AND
            AL.PAGENAME LIKE '%PAGE%')
        AND
            AL.PAGENAME NOT LIKE '%ERROR%'
        AND
            AL.PAGENAME NOT LIKE '%PAGE_DEMO_INDEX%'
        AND
            UPPER(AL.PAGENAME) NOT LIKE '%CACHE%'
        AND
            AL.IPVALUE <> '::1'
        ORDER BY
            AL.[ID_column] DESC
    ";

    let stream = client.simple_query(sql).await?;
    let rows = stream.into_first_result().await?;

    let mut access_logs = Vec::new();

    for row in rows {
        //let id_column: i64 = row.get("id_column").unwrap_or(0);
        let id_column: i32 = row.get::<i32, _>("id_column").unwrap_or_default();
        let page_name: Option<&str> = row.get("page_name");
        
        //let access_date: Option<&str> = row.get("access_date");
        use chrono::NaiveDateTime;

        let access_date: Option<NaiveDateTime> = row.get("access_date");
        let access_date_str = access_date.map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string());

        let ip_value: Option<&str> = row.get("ip_value");

        access_logs.push(AccessLog {
            id_column,
            page_name: page_name.map(|s| s.to_string()),
            //access_date: access_date.map(|s| s.to_string()),
            access_date: access_date_str,
            ip_value: ip_value.map(|s| s.to_string()),
        });
    }

    Ok(access_logs)
}

pub async fn fetch_persons() -> Result<Vec<PersonaTable>, Box<dyn std::error::Error + Send + Sync>> {
    let mut client = create_db_client().await?;

    let sql = "
        SELECT
             [Id_Column]            AS id_column
            ,[Ciudad]              AS ciudad
            ,[NombreCompleto]      AS nombre_completo
        FROM
            [dbo].[Persona]
        ORDER BY
            Id_Column ASC
    ";

    let stream = client.simple_query(sql).await?;
    let rows = stream.into_first_result().await?;

    let mut personas = Vec::new();

    for row in rows {
        //let id_column: i64 = row.get("id_column").unwrap_or(0);
        let id_column: i32                = row.get::<i32, _>("id_column").unwrap_or_default();
        let ciudad: Option<&str>          = row.get("ciudad");
        let nombre_completo: Option<&str> = row.get("nombre_completo");

        personas.push(PersonaTable {
            id_column,
            ciudad: ciudad.map(|s| s.to_string()),
            nombre_completo: nombre_completo.map(|s| s.to_string()),
        });
    }

    Ok(personas)
}