import * as React from 'react';
import * as ReactDOM from 'react-dom';
import './index.css'
import App from "./App";
import {createTheme, initializeIcons, mergeStyles, ThemeProvider} from '@fluentui/react';
import i18n from "i18next";
import {initReactI18next} from "react-i18next";
import {change_lang, get_lang} from "./utils";
import i18next from "i18next";
import Theme from "./Theme";


initializeIcons()
// Inject some global styles
mergeStyles({
    ':global(body,html,#root)': {
        margin: 0,
        padding: 0,
        height: '100vh',
    },
});
let _ = i18n
    .use(initReactI18next)
    .init({
        resources: {
            "en": {
                translation: {
                    "setting-header": "Setting",
                    "exclude-path-label":"Exclude Path",
                    "add":"Add",
                    "all":"All",
                    "photo":"Photo",
                    "video":"Video",
                    "document":"Document",
                    "folder":"Folder",
                    "name":"Name",
                    "last-modified":"Last Modified",
                    "size":"Size",
                    "path":"Path",
                    "lang":"Language",
                    "theme":"Theme",
                    "theme-default":"Default",
                    "theme-light-purple":"Light Purple",
                    "theme-light-blue":"Light Blue",
                    "reindex":"Reindex",
                    "reindex-dialog":"Do you want to Reindex? It will take effect on next reboot!",
                    "remove":"Remove",
                    "confirm":"Confirm",
                    "cancel":"Cancel",
                    "add_exclude_path_err":"Invalid path",
                    "upgrade":"Upgrade",
                    "version":"Version V",
                    "rmenu-open":"Open",
                    "rmenu-copy-path":"Copy Path",
                    "rmenu-open-in-terminal":"Open in Terminal",
                    "rmenu-open-in-explorer":"Open in Explorer",
                    "rmenu-automation-basic_expense": "Basic Expense Process",
                    "rmenu-automation-project_statistic": "Project Statistic Process",
                    "rmenu-config-template-basic_expense": "Config As Basic Expense Template",
                    "rmenu-config-template-basic_expense-again": "Config As Basic Expense Template Again",
                    "rmenu-config-template-project_statistic": "Config As Project Statistic Template",
                    "rmenu-config-template-project_statistic-again": "Config As Project Statistic Template Again",
                    "progress":"Progress: ",
                    "file-indexed":"File indexed: ",
                }
            },
            "zh-CN":{
                translation: {
                    "setting-header": "??????",
                    "exclude-path-label":"????????????",
                    "add":"??????",
                    "all":"??????",
                    "photo":"??????",
                    "video":"??????",
                    "document":"EXCEL??????",
                    "folder":"?????????",
                    "name":"??????",
                    "last-modified":"????????????",
                    "size":"??????",
                    "path":"??????",
                    "lang":"??????",
                    "theme":"??????",
                    "theme-default":"??????",
                    "theme-light-purple":"?????????",
                    "theme-light-blue":"?????????",
                    "reindex":"?????????",
                    "reindex-dialog":"??????????????????????????????????????????????????????????????????",
                    "remove":"??????",
                    "confirm":"??????",
                    "cancel":"??????",
                    "add_exclude_path_err":"????????????",
                    "upgrade":"??????",
                    "version":"?????? V",
                    "rmenu-open":"??????",
                    "rmenu-copy-path":"????????????",
                    "rmenu-automation-basic_expense": "???????????????????????????",
                    "rmenu-automation-project_statistic": "????????????????????????",
                    "rmenu-config-template-basic_expense": "????????????????????????????????????",
                    "rmenu-config-template-basic_expense-again": "???????????????????????????????????????",
                    "rmenu-config-template-project_statistic": "?????????????????????????????????",
                    "rmenu-config-template-project_statistic-again": "????????????????????????????????????",
                    "rmenu-open-in-terminal":"Terminal??????",
                    "rmenu-open-in-explorer":"?????????????????????",
                    "progress":"??????: ",
                    "file-indexed":"???????????????: ",
                }
            }
        },
        lng: "en",
        fallbackLng: "en",
        interpolation: {
            escapeValue: false
        }
    });






ReactDOM.render(<>
    <Theme/>
</>, document.getElementById('root'));



