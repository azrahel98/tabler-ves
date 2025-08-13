import path from "path";
import fs from "fs";
import minimist from "minimist";
import puppeteer from "puppeteer";

async function generarPDF(nombre: string, mes: string, monto: string) {
  const isPkg = typeof (process as any).pkg !== "undefined";

  const htmlPath = isPkg
    ? path.join(path.dirname(process.execPath), "boleta.html")
    : path.join(__dirname, "boleta.html");

  if (!fs.existsSync(htmlPath)) {
    throw new Error(`No se encontró el archivo HTML: ${htmlPath}`);
  }

  let html = fs.readFileSync(htmlPath, "utf8");
  html = html
    .replace(/{{nombre}}/g, nombre)
    .replace(/{{mes}}/g, mes)
    .replace(/{{monto}}/g, monto);

  const browser = await puppeteer.launch({
    headless: true,
    args: ["--no-sandbox", "--disable-setuid-sandbox"],
  });
  const page = await browser.newPage();
  await page.setContent(html, { waitUntil: "networkidle0" });

  const pdfPath = path.join(process.cwd(), `boleta-${nombre}-${mes}.pdf`);

  await page.emulateMediaType("print");

  await page.pdf({
    path: "boleta.pdf",
    format: "A4",
    printBackground: true,
    scale: 1,
    margin: { top: 0, right: 0, bottom: 0, left: 0 },
  });

  await browser.close();
  console.log(`PDF generado: ${pdfPath}`);
}

function main() {
  const args = minimist(process.argv.slice(2));
  const nombre = args.nombre || "Empleado";
  const mes = args.mes || "2025-08";
  const monto = args.monto || "0.00";

  generarPDF(nombre, mes, monto).catch((err) => {
    console.error("Error generando PDF:", err);
    process.exit(1);
  });
}

main();

// pkg dist/index.js --targets node18-win-x64,node18-linux-x64 --assets dist/boleta.html
// npx tsc
// bun i -g pkg
