export default class Sidebar {
    constructor() {
        this._submitButton = document.querySelector('.sidebar .button.submit');
        this._fileBrowseButton = document.querySelector('.sidebar .file-browse');
        this._fileContainer = document.querySelector('.sidebar .file-container');
        this._fileInput = document.querySelector('.sidebar input[name="file"]');

        console.log(this);
    }
}