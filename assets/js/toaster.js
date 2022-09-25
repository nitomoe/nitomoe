/**
 * @typedef { 'info' | 'success' | 'error' } ToasterType
 * @typedef { {info: ToasterType, timeout: Number }} ToasterOptions
 */

 class ToasterMessage {
    /**
     * Constructs a new ToasterMessage object
     * 
     * @param {ToasterType} type
     * @param {String} message 
     * @param {Number} timeout 
     * @param {HTMLDivElement} container
     */
    constructor(type, message, timeout, container) {
        this.type = type;
        this.message = message;
        this.timeout = timeout;
        this.container = container;

        const messageText = document.createElement('div');
        messageText.style.padding = '15px';
        messageText.innerText = message;

        this._messageDiv = document.createElement('div');
        switch (this.type) {
            case 'info':
                this._messageDiv.style.color = '#000';
                this._messageDiv.style.backgroundColor = '#fff';
            break;
            case 'error':
                this._messageDiv.style.color = '#fff';
                this._messageDiv.style.backgroundColor = '#ff5959';
            break;
            case 'success':
                this._messageDiv.style.color = '#000';
                this._messageDiv.style.backgroundColor = '#59ff59';
            break;
        }
        this._messageDiv.style.width = '100%';
        this._messageDiv.style.marginBottom = '15px';
        this._messageDiv.style.marginTop = '15px';
        this._messageDiv.style.borderRadius = '4px';
        this._messageDiv.style.lineHeight = '1.5em';
        this._messageDiv.style.boxShadow = '0 0 3px 1px #000';
        this._messageDiv.style.transition = 'opacity 0.5s linear, margin-top 0.5s linear';
        
        this._messageDiv.addEventListener('mouseenter', (event) => {
            const target = event.currentTarget;
            target.style.cursor = 'pointer';
        });

        this._messageDiv.addEventListener('mouseleave', (event) => {
            const target = event.currentTarget;
            target.style.cursor = 'auto';
        });

        this._messageDiv.addEventListener('click', () => {
            progressBar.style.animationPlayState = 'paused';
            this.remove();
        });

        const progressBarContainer = document.createElement('div');
        progressBarContainer.setAttribute('style', `--duration: ${this.timeout / 1000}`);

        const progressBar = document.createElement('div');
        progressBar.style.height = '5px';
        progressBar.style.backgroundColor = '#eee';
        progressBar.style.animation = 'roundtime calc(var(--duration) * 1s) linear forwards';
        progressBar.style.transformOrigin = 'left center';
        progressBar.style.opacity = 0.5;
        progressBarContainer.appendChild(progressBar);

        this._messageDiv.appendChild(messageText);
        this._messageDiv.appendChild(progressBarContainer);

        const resizeMessageDivCallback = (matches) => {
            if (matches) {
                messageText.style.textAlign = 'center';
                container.style.width = '90%';
            }
            else {
                messageText.style.textAlign = 'left';
                container.style.width = '550px';
            }
        };

        const resizeMessageDiv = matchMedia('(max-width: 550px), ');
        resizeMessageDiv.addEventListener('change', event => resizeMessageDivCallback(event.matches));
        resizeMessageDivCallback(resizeMessageDiv.matches);

        this.container.insertBefore(this._messageDiv, this.container.firstChild);
        
        progressBar.addEventListener('animationend', this.remove.bind(this));
    }

    /**
     * Removes the Toaster Message
     */
    remove() {
        this._messageDiv.style.opacity = 0;
        this._messageDiv.style.marginTop = '65px';
        
        this._messageDiv.addEventListener('transitionend', (event) => {
            if (event.propertyName == 'opacity') {
                this.container.removeChild(this._messageDiv);
            }
        });
    }
};

export default class Toaster {
    /**
     * Constructs a new Toaster object
     * 
     * @param {ToasterOptions} options 
     */
    constructor(options) {
        const defaultOptions = {
            type: 'info',
            timeout: 2500
        };
        this.options = {...defaultOptions, ...options};
    }

    /**
     * Presents a Toaster message to the screen
     * 
     * @param {String} message 
     */
    show(message) {
        let toasterContainer = document.querySelector('.toaster-container');

        if (!toasterContainer) {
            toasterContainer = document.createElement('div');
            toasterContainer.className = 'toaster-container';
            toasterContainer.style.position = 'fixed';
            toasterContainer.style.top = 0;
            toasterContainer.style.right = 0;
            toasterContainer.style.width = '550px';
            toasterContainer.style.marginRight = '15px';
            toasterContainer.style.zIndex = 9999;
            document.body.appendChild(toasterContainer);

            const toasterStyle = document.createElement('style');
            toasterStyle.type = 'text/css';
            toasterStyle.innerHTML = `@keyframes roundtime {
                to {
                  transform: scaleX(0);
                }
              }`;

            document.head.appendChild(toasterStyle);
        }

        new ToasterMessage(this.options.type, message, this.options.timeout, toasterContainer);
    }
}