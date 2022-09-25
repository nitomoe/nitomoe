import Toaster from './toaster.js'

export default class Sidebar {
    constructor() {
        this._submitButton = document.querySelector('.sidebar .button.submit');
        this._fileBrowseButton = document.querySelector('.sidebar .file-browse');
        this._fileContainer = document.querySelector('.sidebar .file-container');
        this._fileInput = document.querySelector('.sidebar input[name="file"]');

        this._submitButton.addEventListener('click', () => {
            new Toaster({type: 'error'}).show("click click click");
        })
    }
}