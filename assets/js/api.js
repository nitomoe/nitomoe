export class api {
    /**
     * Preforms an API fetch
     *
     * @typedef {Promise<Response>}
     * @param {String} endpoint
     * @param {RequestInit} options
     *
     */
    static async fetch(endpoint, options = {}) {
      return await fetch(`/api/v1/${endpoint}`, options);
    }
  
    /**
     * Preforms an authenticated API fetch
     *
     * @typedef {Promise<Response>}
     * @param {String} endpoint
     * @param {RequestInit} options
     *
     */
    static async authFetch(endpoint, options = {}) {
      options = Object.assign({
        headers: {
          Authorization: `Bearer ${token}`
        }
      }, options);
  
      return await api.fetch(endpoint, options);
    }
  
    /**
     * Preforms a standard API get request
     *
     * @typedef {Promise<Response>}
     * @param {String} endpoint
     * @param {RequestInit} options
     *
     */
    static async get(endpoint, options = {}) {
      return await api.fetch(endpoint, options);
    }
  
    /**
     * Preforms a standard API post request
     *
     * @typedef {Promise<Response>}
     * @param {String} endpoint
     * @param {RequestInit} options
     *
     */
    static async post(endpoint, options = {}) {
      options = Object.assign({
        method: 'POST'
      }, options);
  
      return await api.fetch(endpoint, options);
    }
  
  
    /**
     * Preforms a standard API upload post request
     *
     * @typedef {Promise<Response>}
     * @param {String} endpoint
     * @param {Object} options
     */
    static async upload(endpoint, options = {}) {
      return await new Promise((resolve, reject) => {
        const xhr = new XMLHttpRequest();
        xhr.open('POST', `/api/v1/${endpoint}`);
  
        if (options.onUploadProgress) {
          xhr.upload.addEventListener('progress', event => options.onUploadProgress(event));
        }
  
        xhr.onreadystatechange = () => {
          if (xhr.readyState === 4) {
            resolve(JSON.parse(xhr.response));
          }
        };
  
        xhr.onerror = error => reject(error);
  
        xhr.send(options.body);
      });
    }
  }
  