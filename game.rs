 scene.setBackgroundColor(0)//color de fondo
 let mySprite: Sprite = null;//crear el jugador
 scene.setBackgroundColor(0)//cambio de fondo 
 mySprite = sprites.create(img`
     . . . . . . f f f f . . . . . .
     . . . . f f f 2 2 f f f . . . .
     . . . f f f 2 2 2 2 f f f . . .
     . . f f f e e e e e e f f f . .
     . . f f e 2 2 2 2 2 2 e e f . .
     . . f e 2 f f f f f f 2 e f . .
     . . f f f f e e e e f f f f . .
     . f f e f b f 4 4 f b f e f f .
     . f e e 4 1 f d d f 1 4 e e f .
     . . f e e d d d d d d e e f . .
     . . . f e e 4 4 4 4 e e f . . .
     . . e 4 f 2 2 2 2 2 2 f 4 e . .
     . . 4 d f 2 2 2 2 2 2 f d 4 . .
     . . 4 4 f 4 4 5 5 4 4 f 4 4 . .
     . . . . . f f f f f f . . . . .
     . . . . . f f . . f f . . . . .
 `,SpriteKind.Player)//crear un sprite para el jugador
controller.moveSprite(mySprite);
let pizza = sprites.create(img`
    . . . . . . . . . . . . . . . .
    . . . . . . . . . . . . . . . .
    . . . . . . . . . . . . . . . .
    . . . . . . . . . . . . . . . .
    . . . . . . 8 8 8 8 . . . . . .
    . . . . 8 8 8 8 8 8 8 . . . . .
    . . . . 8 8 8 8 8 8 8 8 . . . .
    . . . . 8 8 8 8 8 8 8 8 . . . .
    . . . . 8 8 8 8 8 8 8 8 . . . .
    . . . . . 8 8 8 8 8 8 8 . . . .
    . . . . . . 8 8 8 8 8 . . . . .
    . . . . . . . . . . . . . . . .
    . . . . . . . . . . . . . . . .
    . . . . . . . . . . . . . . . .
    . . . . . . . . . . . . . . . .
    . . . . . . . . . . . . . . . .
`, SpriteKind.Food);
sprites.onOverlap(SpriteKind.Player, SpriteKind.Food, function (sprite, otherSprite){
    info.changeScoreBy(1);
    otherSprite.setPosition(randint(0,160),randint (0,120));
    info.startCountdown(10);
})