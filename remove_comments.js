const fs = require('fs');
const path = require('path');

function removeComments(code, ext) {
    if (ext === '.css') {
        return code.replace(/\/\*[\s\S]*?\*\//g, '');
    }
    
    let inString = false;
    let stringChar = '';
    let inMultilineComment = false;
    let inSinglelineComment = false;
    let result = '';
    
    for (let i = 0; i < code.length; i++) {
        const char = code[i];
        const nextChar = code[i + 1];
        
        if (inMultilineComment) {
            if (char === '*' && nextChar === '/') {
                inMultilineComment = false;
                i++;
            }
            continue;
        }
        
        if (inSinglelineComment) {
            if (char === '\n' || char === '\r') {
                inSinglelineComment = false;
                result += char;
            }
            continue;
        }
        
        if (inString) {
            result += char;
            if (char === '\\') {
                result += nextChar;
                i++;
            } else if (char === stringChar) {
                inString = false;
            }
            continue;
        }
        
        if (char === '"' || char === "'" || (ext !== '.rs' && char === '`')) {
            inString = true;
            stringChar = char;
            result += char;
            continue;
        }
        
        if (char === '/' && nextChar === '*') {
            inMultilineComment = true;
            i++;
            continue;
        }
        
        if (char === '/' && nextChar === '/') {
            inSinglelineComment = true;
            i++;
            continue;
        }
        
        if (ext === '.vue' && char === '<' && code.slice(i, i + 4) === '<!--') {
            let j = i + 4;
            while (j < code.length && code.slice(j, j + 3) !== '-->') {
                j++;
            }
            i = j + 2;
            continue;
        }

        result += char;
    }
    
    return result;
}

function processDir(dir) {
    const files = fs.readdirSync(dir);
    for (const file of files) {
        const fullPath = path.join(dir, file);
        if (fs.statSync(fullPath).isDirectory()) {
            processDir(fullPath);
        } else {
            const ext = path.extname(file);
            if (['.rs', '.ts', '.vue', '.css'].includes(ext)) {
                console.log(`Processing ${fullPath}...`);
                const content = fs.readFileSync(fullPath, 'utf8');
                const cleaned = removeComments(content, ext);
                fs.writeFileSync(fullPath, cleaned, 'utf8');
            }
        }
    }
}

const args = process.argv.slice(2);
args.forEach(processDir);
