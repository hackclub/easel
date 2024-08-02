

export class MeowError extends Error{
    constructor(msg) {
        super()
        this.message = msg
    }

    toString(){
        return this.message
    }
}

export default {
    grrr: args => console.log(...args),
    randmeow: ([min, max]) => {
        if (min >= 0 && max <= 1) return Math.random()
        return Math.random() * (max - min + 1) + min
    },
    round: number => Math.round(number),
    meows: () => {
        let outs = "";
        for (let i = 0; i < Math.random() * (100 - 20 + 1) + 20; i++){
            outs = "";
            for (let j = 0; j < Math.random() * (30 - 1 + 1) + 1; j++)
                outs = outs + "meow "
            console.log(outs)
        }       
    },
    catFind: ([inS, inC]) => {
        let text = new String(inS);
        let textn = new String(inC);
        for (let i = 0; i < text.length; i++) {
            if(text.charAt(i) == textn) {
                return i;
            }
        }
        return false;
    },
    catReplace: ([inS, inO, inN]) => {
        let text = new String(inS)
        let inOld = new String(inO)
        let inNew = new String(inN)
        return text.replace(inOld, inNew)
    },
    catReplaceAll: ([inS, inO, inN]) => {
        let text = new String(inS)
        let inOld = new String(inO)
        let inNew = new String(inN)
        return text.replaceAll(inOld, inNew)
    },
    meowupper: (inS) => {
        let text = new String(inS)
        return text.toUpperCase()
    },
    meowlower: (inS) => {
        let text = new String(inS)
        return text.toLowerCase()
    },
    meowclean: (inS) => {
        let text = new String(inS)
        text = text.toUpperCase()
        let letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
        let out = ""
        for(let i = 0; i < text.length; i++) {
            if (letters.indexOf(text.charAt(i)) == -1){
                continue
            }
            out += text[i]
        }
        return out
    }
}