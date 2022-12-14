import {invoke} from "@tauri-apps/api";
import {isPermissionGranted, requestPermission, sendNotification} from "@tauri-apps/api/notification";
import {BaseDirectory, createDir, readTextFile, writeFile} from "@tauri-apps/api/fs";

const fileType2ext = {
    4: "bmp jpg gif png jpeg",
    3: "mp4 mov avi flv f4v mkv",
    2: "doc txt pdf ppt pptx docx xlsx xls",
    5: "xlsx xls"
}

export async function suggest(kw) {
    return await invoke('suggest', {
        kw: kw
    });
}

export async function search(kw, no) {
    // let ext = fileType2ext[no];
    let ext = "xlsx xls"
    let dirOpt = undefined;
    if (no !== undefined) {
        if (no === '1') {
            dirOpt = true;
        }
    }
    return await invoke('search', {
        kw: kw,
        isDirOpt: dirOpt,
        extOpt: ext,
    });
}

export function open_file_location_in_terminal(row) {
    let _ = invoke('open_file_in_terminal', {
        kw: row.abs_path
    });
}

export function open_file_location_in_explorer(row) {
    let _ = invoke('open_file_in_explorer', {
        kw: row.abs_path
    });
}

const TEMPLATE_PATHS = {
    "basic": 'data/basic_expense_template.json',
    "project": 'data/project_statistic_template.json'
}

const TEMPLATE_TYPES = {
    "basic": "基本支出情况",
    "project": "项目收支统计"
}

export async function get_exist_template_configs(type) {
    try {
        let template_path = await readTextFile(
            TEMPLATE_PATHS[type],
            {dir: BaseDirectory.App}
        );
        return template_path
    } catch (e) {
        return null
    }
}

export async function config_template_path(row, type) {
    let exist_templates = await get_exist_template_configs(type)
    if (!exist_templates) {
        await createDir("data", {
            dir: BaseDirectory.App,
            recursive: true,
        });
    }
    console.log(row, type)
    await writeFile(
        {
            contents: row.abs_path,
            path: TEMPLATE_PATHS[type],
        },
        {dir: BaseDirectory.App}
    );
    let new_templates = await get_exist_template_configs(type)
    let info = "已经设置" + TEMPLATE_TYPES[type] + "模版地址为: " + new_templates
    console.log(info);
    await send_notification(TEMPLATE_TYPES[type] + '模版设置成功！', "已经设置" + TEMPLATE_TYPES[type] + "模版地址为: " + row.abs_path);
}

async function send_notification(title, body) {
    let permissionGranted = await isPermissionGranted();
    if (!permissionGranted) {
        const permission = await requestPermission();
        permissionGranted = permission === 'granted';
    }
    if (permissionGranted) {
        sendNotification({title: title, body: body});
    }
}

export async function excel_automation(row, type) {
    let exist_templates = await get_exist_template_configs(type)
    if (!exist_templates) {
        console.error('请先设置自动化处理模版！')
        await send_notification('模版未设置！', '请先设置自动化处理模版！')

    } else {
        console.log(row.abs_path, exist_templates)
        await send_notification('开始' + TEMPLATE_TYPES[type] +'自动化处理!', '处理路径: ' + row.abs_path + '\n' + '使用模版:' + exist_templates);
        let execute_result = await invoke('excel_automation_backend', {
            // kw: row.abs_path
            filePath: row.abs_path,
            templatePath: exist_templates
        });
        console.log("execute_result", execute_result);
    }
}

export function open_file_location(row) {
    let _ = invoke('open_file_path', {
        kw: row.abs_path
    });
}

export async function get_lang() {
    return await invoke('get_lang', {});
}

export function change_lang(lang) {
    let _ = invoke('change_lang', {
        lang: lang
    });

}

export async function walk_metrics() {
    return await invoke('walk_metrics', {});

}

export async function get_theme() {
    return await invoke('get_theme', {});
}

export function change_theme(theme) {
    let _ = invoke('change_theme', {
        theme: theme
    });
}

export async function add_exclude_path(path) {

    return await invoke('add_exclude_path', {
        path: path
    });
}

export async function remove_exclude_path(path) {
    return await invoke('remove_exclude_path', {
        path: path
    });
}

export function upgrade() {
    let _ = invoke('upgrade', {});
}

export function reindex() {
    let _ = invoke('reindex', {});
}

export async function get_exclude_paths() {
    return await invoke('get_exclude_paths', {});
}