/** utils.js from https://github.com/loks0n MIT License - Copyright (c) 2023 Appwrite  */

import path from 'path';
import { fileURLToPath } from 'url';
import fs from 'fs';

/**
 * Throws an error if any of the keys are missing from the object
 * @param {*} obj
 * @param {string[]} keys
 * @throws {Error}
 */
export function throwIfMissing(obj, keys) {
  const missing = [];
  for (let key of keys) {
    if (!(key in obj) || !obj[key]) {
      missing.push(key);
    }
  }
  if (missing.length > 0) {
    throw new Error(`Missing required fields: ${missing.join(', ')}`);
  }
}

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const staticFolder = path.join(__dirname, '../static');

/**
 * Returns the contents of a file in the static folder
 * @param {string} fileName
 * @returns {string} Contents of static/{fileName}
 */
export function getStaticFile(fileName) {
  return fs.readFileSync(path.join(staticFolder, fileName)).toString();
}

/**
 * @param {string} template
 * @param {Record<string, string | undefined>} values
 * @returns {string}
 */
export function interpolate(template, values) {
    return template.replace(/{{([^}]+)}}/g, (_, key) => values[key] || '');
  }