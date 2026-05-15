#[derive(Debug, Clone)]
struct Vuelo {
    id: String,
    altitud: u32, // Este será nuestra clave (key)
}
struct Nodo {
    vuelo: Vuelo,
    // Box<Nodo> da un tamaño conocido en compilación (puntero),
    // evitando un tipo recursivo de tamaño infinito.
    izquierdo: Option<Box<Nodo>>,
    derecho: Option<Box<Nodo>>,
    altura: i32,
}
impl Nodo {
    fn nuevo(vuelo: Vuelo) -> Self {
        Nodo {
            vuelo,
            izquierdo: None,
            derecho: None,
            altura: 1,
        }
    }
}
// --- UTILIDADES DE BALANCEO (NO MODIFICAR) ---
fn obtener_altura(nodo: &Option<Box<Nodo>>) -> i32 {
    nodo.as_ref().map_or(0, |n| n.altura)
}
fn actualizar_altura(nodo: &mut Nodo) {
    nodo.altura = 1 + std::cmp::max(
        obtener_altura(&nodo.izquierdo),
        obtener_altura(&nodo.derecho),
    );
}
fn obtener_balance(nodo: &Nodo) -> i32 {
    obtener_altura(&nodo.izquierdo) - obtener_altura(&nodo.derecho)
}
fn rotar_derecha(mut y: Box<Nodo>) -> Box<Nodo> {
    // take() mueve/transfiere ownership de y.izquierdo fuera de y,
    // y deja None en su lugar para mantener a y en un estado válido.
    let mut x = y.izquierdo.take().expect("Error de radar");
    // Aquí también se mueve el subárbol derecho de x hacia y.izquierdo.
    y.izquierdo = x.derecho.take();
    actualizar_altura(&mut y);
    // Luego y se mueve dentro de Some(y), pasando a ser hijo derecho de x.
    x.derecho = Some(y);
    actualizar_altura(&mut x);
    x
}
fn rotar_izquierda(mut x: Box<Nodo>) -> Box<Nodo> {
    // take() evita clonar nodos: solo reubica propietarios de subárboles.
    let mut y = x.derecho.take().expect("Error de radar");
    x.derecho = y.izquierdo.take();
    actualizar_altura(&mut x);
    y.izquierdo = Some(x);
    actualizar_altura(&mut y);
    y
}
// --- FUNCIÓN DE INSERCIÓN ---
fn insertar(nodo_opt: Option<Box<Nodo>>, vuelo: Vuelo) -> Box<Nodo> {
    let mut nodo = match nodo_opt {
        None => return Box::new(Nodo::nuevo(vuelo)),
        Some(n) => n,
    };
    if vuelo.altitud < nodo.vuelo.altitud {
        // nodo.izquierdo.take() mueve el hijo izquierdo temporalmente fuera del nodo
        // para ceder ownership a la llamada recursiva y luego reasignarlo.
        nodo.izquierdo = Some(insertar(nodo.izquierdo.take(), vuelo.clone())); // BORRAR EL .CLONE
    } else if vuelo.altitud > nodo.vuelo.altitud {
        nodo.derecho = Some(insertar(nodo.derecho.take(), vuelo.clone())); // BORRAR EL .CLONE
    } else {
        return nodo;
    }
    actualizar_altura(&mut nodo);
    let balance = obtener_balance(&nodo);
    // Caso Izquierda-Izquierda
    if balance > 1 && vuelo.altitud < nodo.izquierdo.as_ref().unwrap().vuelo.altitud {
        return rotar_derecha(nodo);
    }
    // Caso Derecha-Derecha
    if balance < -1 && vuelo.altitud > nodo.derecho.as_ref().unwrap().vuelo.altitud {
        return rotar_izquierda(nodo);
    }
    // Caso Izquierda-Derecha
    if balance > 1 && vuelo.altitud > nodo.izquierdo.as_ref().unwrap().vuelo.altitud {
        // Se toma ownership del hijo para rotarlo y volver a conectarlo.
        let hijo_izq = nodo.izquierdo.take().unwrap();
        nodo.izquierdo = Some(rotar_izquierda(hijo_izq));
        return rotar_derecha(nodo);
    }
    // Caso Derecha-Izquierda
    if balance < -1 && vuelo.altitud < nodo.derecho.as_ref().unwrap().vuelo.altitud {
        let hijo_der = nodo.derecho.take().unwrap();
        nodo.derecho = Some(rotar_derecha(hijo_der));
        return rotar_izquierda(nodo);
    }
    nodo
}

// FASE 2: Localización de Vuelos (60 min)
fn buscar_vuelo(nodo: &Option<Box<Nodo>>, altitud: u32) -> Option<&Vuelo> {
    // Evaluamos en qué estado está el nodo actual usando referencias
    match nodo {
        Some(n) => {
            if altitud == n.vuelo.altitud {
                // Si lo encontre, se devulve una referencia al vuelo
                Some(&n.vuelo)
            } else if altitud < n.vuelo.altitud {
                // El vuelo buscado está más abajo, buscamos en la rama izquierda
                buscar_vuelo(&n.izquierdo, altitud)
            } else {
                // El vuelo buscado está más arriba, buscamos en la rama derecha
                buscar_vuelo(&n.derecho, altitud)
            }
        }
        None => {
            // Llegamos a una hoja vacía y no lo encontramos
            None
        }
    }
}

fn main() {
    let mut radar: Option<Box<Nodo>> = None;
    // Simulación de entrada de vuelos
    let datos = vec![
        ("AV123", 5000), ("UA456", 3000), ("IB101", 2000),
        ("AF999", 4000), ("TA222", 3500), ("AM777", 6000),
    ];
    for (id, alt) in datos {
        let v = Vuelo { id: id.to_string(), altitud: alt };
        radar = Some(insertar(radar.take(), v));
    }

    /*
    Logica de Insercion de los datos [5000, 3000, 2000, 4000, 3500, 6000]
    1. Insertar 5000: Raíz del árbol.

               [5000]

    2. Insertar 3000: Va a la izquierda de 5000.

                [5000]
               /
           [3000]

    3. Insertar 2000: Va a la izquierda de 3000,
       *causando un desequilibrio (balance = 2).

                [5000]
               /
           [3000]
           /
       [2000]

       *Se realiza una rotación simple a la derecha en 5000.

                [3000]
               /      \
           [2000]      [5000]

    4. Insertar 4000: Va a la derecha de 3000, Y a la izquierda de 5000

                [3000]
               /      \
           [2000]     [5000]
                       /
                    [4000]

    5. Insertar 3500: Va a la izquierda de 4000,
       *causando un desequilibrio (balance = 2).

                [3000]
               /      \
           [2000]     [5000]
                       /
                    [4000]
                    /
                 [3500]

       *Se realiza una rotación simple a la derecha en 5000.

                [3000]
               /      \
           [2000]     [4000]
                      /    \
                   [3500]  [5000]

    6. Insertar 6000: Va a la derecha de 5000
       * Desequilibrio en 3000 (Raíz): Balance de -2. Hijo (4000) peso a la derecha.
       * Caso Derecha-Derecha.

                 [3000]
               /      \
           [2000]     [4000]
                      /    \
                   [3500]  [5000]
                                \
                                [6000]

       *Se realiza una rotación simple a la izquierda sobre la raiz 3000.

                 [4000]
                /      \
           [3000]      [5000]
           /    \            \
      [2000]    [3500]        [6000]


    */

    println!("--- Radar de Control Aéreo (AVL) ---");
    // Aquí el estudiante debe invocar sus funciones de búsqueda y eliminación


    // FASE 2: Inserción de Vuelos (30 min)
    println!("\n--- FASE 2: Búsqueda de Vuelos ---");

    // 1. Buscar un vuelo existente
    let altitud_buscada = 4000;
    match buscar_vuelo(&radar, altitud_buscada) {
        Some(v) => println!("¡¡Alerta Radar!! Vuelo {} detectado a {} pies.", v.id, v.altitud),
        None => println!("Espacio aéreo despejado a {} pies.", altitud_buscada),
    }

    // 2. Buscar un vuelo que no exista
    let altitud_falsa = 9999;
    match buscar_vuelo(&radar, altitud_falsa) {
        Some(v) => println!("¡¡Alerta Radar!! Vuelo {} detectado a {} pies.", v.id, v.altitud),
        None => println!("Espacio aéreo despejado a {} pies.", altitud_falsa),
    }

}
