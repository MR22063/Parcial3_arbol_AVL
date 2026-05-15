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

// FASE 2: Localización de Vuelos
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

// FASE 3: Descenso y Aterrizaje (Eliminación)

// Función auxiliar para buscar el "predecesor in-order"
// (el nodo con la altitud más alta dentro del subárbol izquierdo)
fn encontrar_maximo(nodo: &Option<Box<Nodo>>) -> Vuelo {
    let mut actual = nodo;
    // Navegamos hacia la derecha hasta que no haya más nodos
    while let Some(n) = actual {
        if n.derecho.is_none() {
            // Clonamos el vuelo porque solo queremos copiar sus datos,
            // no extraer el nodo completo de la memoria aún.
            return n.vuelo.clone();
        }
        actual = &n.derecho;
    }
    unreachable!("Error del radar: Intento de buscar máximo en un árbol vacío.");
}

fn eliminar_vuelo(nodo_opt: Option<Box<Nodo>>, altitud: u32) -> Option<Box<Nodo>> {
    // Si llegamos a un nodo vacío, simplemente retornamos None (caso base)
    let mut nodo = match nodo_opt {
        Some(n) => n,
        None => return None,
    };

    // 1. Busqueda y Eliminacion
    if altitud < nodo.vuelo.altitud {
        nodo.izquierdo = eliminar_vuelo(nodo.izquierdo.take(), altitud);
    } else if altitud > nodo.vuelo.altitud {
        nodo.derecho = eliminar_vuelo(nodo.derecho.take(), altitud);
    } else {
        // Se encontro el avion (altitud == nodo.vuelo.altitud)
        // Caso 1: No tiene hijo izquierdo (o es una hoja sin hijos)
        if nodo.izquierdo.is_none() {
            return nodo.derecho.take();
        }
        // Caso 2: No tiene hijo derecho
        else if nodo.derecho.is_none() {
            return nodo.izquierdo.take();
        }
        // Caso 3: Tiene DOS hijos (El gran desafío)
        else {
            // Buscamos el predecesor in-order (el máximo del subárbol izquierdo)
            let predecesor = encontrar_maximo(&nodo.izquierdo);

            // Reemplazamos los datos del nodo actual con los del predecesor
            nodo.vuelo = predecesor.clone();

            // Ahora eliminamos el nodo original del predecesor que quedó duplicado abajo
            nodo.izquierdo = eliminar_vuelo(nodo.izquierdo.take(), predecesor.altitud);
        }
    }

    // 2. RE-BALANCEO (Post-Aterrizaje)
    actualizar_altura(&mut nodo);
    let balance = obtener_balance(&nodo);

    // Caso Izquierda-Izquierda
    if balance > 1 && obtener_balance(nodo.izquierdo.as_ref().unwrap()) >= 0 {
        return Some(rotar_derecha(nodo));
    }
    // Caso Izquierda-Derecha
    if balance > 1 && obtener_balance(nodo.izquierdo.as_ref().unwrap()) < 0 {
        let hijo_izq = nodo.izquierdo.take().unwrap();
        nodo.izquierdo = Some(rotar_izquierda(hijo_izq));
        return Some(rotar_derecha(nodo));
    }
    // Caso Derecha-Derecha
    if balance < -1 && obtener_balance(nodo.derecho.as_ref().unwrap()) <= 0 {
        return Some(rotar_izquierda(nodo));
    }
    // Caso Derecha-Izquierda
    if balance < -1 && obtener_balance(nodo.derecho.as_ref().unwrap()) > 0 {
        let hijo_der = nodo.derecho.take().unwrap();
        nodo.derecho = Some(rotar_derecha(hijo_der));
        return Some(rotar_izquierda(nodo));
    }

    // Si el árbol sigue balanceado, devolvemos el nodo tal cual
    Some(nodo)
}

// FASE 4: Alerta de Colisión
fn vuelos_en_rango(nodo: &Option<Box<Nodo>>, min: u32, max: u32) -> usize {
    match nodo {
        Some(n) => {
            let mut contador = 0;
            let altitud = n.vuelo.altitud;

            // Si el vuelo actual está dentro del rango, lo sumamos
            if altitud >= min && altitud <= max {
                contador += 1;
            }

            // Si la altitud actual es MAYOR que el mínimo,
            // significa que aún puede haber vuelos en rango hacia la izquierda.
            if altitud > min {
                contador += vuelos_en_rango(&n.izquierdo, min, max);
            }

            // Si la altitud actual es MENOR que el máximo,
            // significa que aún puede haber vuelos en rango hacia la derecha.
            if altitud < max {
                contador += vuelos_en_rango(&n.derecho, min, max);
            }

            contador
        }
        None => 0,
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


    // FASE 2: Inserción de Vuelos
    println!("\n--- FASE 2: Búsqueda de Vuelos (BUSQUEDA)---");

    // Buscar un vuelo existente
    let altitud_buscada = 4000;
    match buscar_vuelo(&radar, altitud_buscada) {
        Some(v) => println!("¡¡Alerta Radar!! Vuelo {} detectado a {} pies. (VUELO ENCONTRADO)", v.id, v.altitud),
        None => println!("Espacio aéreo despejado a {} pies. (EL VUELO NO EXISTE)", altitud_buscada),
    }

    // Buscar un vuelo que no exista
    let altitud_falsa = 9999;
    match buscar_vuelo(&radar, altitud_falsa) {
        Some(v) => println!("¡¡Alerta Radar!! Vuelo {} detectado a {} pies. (VUELO ENCONTRADO)", v.id, v.altitud),
        None => println!("Espacio aéreo despejado a {} pies. (EL VUELO NO EXISTE)", altitud_falsa),
    }

    // FASE 3: Descenso y Aterrizaje (Eliminación)
    println!("\n--- FASE 3: Aterrizaje de Vuelos (ELIMINACION) ---");

    // Hacemos aterrizar un vuelo que esté en medio del árbol
    let altitud_aterrizaje = 3000;
    println!("Solicitando aterrizaje para el vuelo a {} pies...", altitud_aterrizaje);

    // Eliminamos el vuelo
    radar = eliminar_vuelo(radar.take(), altitud_aterrizaje);
    println!("¡Aterrizaje exitoso! Radar re-balanceado.");

    // Validamos que efectivamente desapareció
    match buscar_vuelo(&radar, altitud_aterrizaje) {
        Some(v) => println!("Error crítico: Vuelo {} sigue en radar a {} pies. (NO SE ELIMINO EL VUELO)", v.id, v.altitud),
        None => println!("Confirmado: El espacio aéreo en {} pies está despejado. (SE ELIMINO EL VUELO EXITOSAMENTE)", altitud_aterrizaje),
    }

    println!("\n FASE 4: Alerta de Colisión (RANGO) ");
    let min_peligro = 3000;
    let max_peligro = 5000;
    let vuelos_peligro = vuelos_en_rango(&radar, min_peligro, max_peligro);
    println!("¡Alerta! Hay {} vuelos en la zona de peligro ({} - {} pies).", vuelos_peligro, min_peligro, max_peligro);

}
