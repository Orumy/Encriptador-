alert ('Solo puedes usar letras minúsculas y números');

let llave = 0;// esta seccion es la llave de desencriptacion
var texto = 'hola tu amigo';//texto a encriptar o desencriptar
let long = 28; // numero de letras 
let i = 8;// inicio del vector de encriptacion 
llave = i;
let j = 0;
let l = 0;
let check = 0;
let clavecr = new Array (28);
//let msg = prompt("Ingresa el texto que quieres procesar");
/*let codigo =    [["a", "b", "c", "d", "e", "f", "g", "h", "i"],
                ["j", "k", "l","m", "n", "ñ", "o", "p", "q"],
                ["r", "s", "t", "u", "v", "w", "x", "y", "z"]];*/




                //encriptador
generadorLlave();
crearCriptex();

let codigo =    ["a", "b", "c", "d", "e", "f", "g", "h", "i","j", "k", "l","m", "n", "ñ", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", " "]; 

for (let cont=0; cont<long; cont++ ) {
    if (i<long){
    clavecr[cont] = codigo[i];
    i++;
    }
        else{
            clavecr[cont] = codigo [j];
            j++;
        } 
}
 console.log(clavecr);
/*
let transcrp =new Array (texto.length);
cont = 0;


    for(l<(texto.length);l++;){
        console.log(texto.length);
        while (check==0){
            if(texto.charAt(cont)==codigo[cont]){
            //compara los valores 
            transcrp[cont]=clavecr[cont];//asigna el nuevo valor
            check=1;
            cont=0;

            }
                else{
                    cont++;
                }   
        } 
        
    }   // no hay valor pero va a la siguiente letra de las 27

    console.log(clavecr);
    console.log(llave);
    console.log(texto.length);
    console.log(transcrp);

    // desencriptador

    */
    function generadorLlave(  ) {
        return Math.floor(Math.random()*28)
        }
        console.log(generadorLlave);



    function crearCriptex ( ){
            for (let cont=0; cont<long; cont++ ) {
                if (i<long){
                clavecr[cont] = codigo[i];
                i++;
                }
                    else{
                        clavecr[cont] = codigo [j];
                        j++;
                    } 
            }
            console.log(clavecr);
    }
       