import React, {useState} from 'react';
import {ContextualMenu, DetailsList, DetailsListLayoutMode, mergeStyleSets, SelectionMode} from "@fluentui/react";
import * as R from "ramda";
import {FileIconType, getFileTypeIconProps, initializeFileTypeIcons} from '@fluentui/react-file-type-icons';
import {Icon} from "office-ui-fabric-react";
import moment from "moment";
import copy from "copy-to-clipboard";
import {config_template_path, excel_automation, get_exist_template_configs, open_file_location_in_explorer} from "./utils";
import {Marker} from "react-mark.js";
import {useTranslation} from "react-i18next";

initializeFileTypeIcons(undefined);


function Items({kw, items, tokenized, setItems}) {

    const {t} = useTranslation();

    let [contextualMenuProps, setContextualMenuProps] = useState();

    function bytesToSize(bytes) {
        const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB'];
        if (bytes === 0) return '0 Byte';
        const i = parseInt(Math.floor(Math.log(bytes) / Math.log(1024)));
        return Math.round(bytes / Math.pow(1024, i), 2) + ' ' + sizes[i];
    }

    function tsFmt(mod_at) {
        return moment.unix(mod_at).format("YYYY/MM/DD");
    }

    function _getKey(item, index) {
        return item.key;
    }

    async function _onItemContextMenu(item, index, ev) {
        let basic_config_menu =
            await get_exist_template_configs('basic') ?
                t("rmenu-config-template-basic_expense-again") :
                t("rmenu-config-template-basic_expense");
        let project_config_menu =
            await get_exist_template_configs('project') ?
                t("rmenu-config-template-project_statistic-again") :
                t("rmenu-config-template-project_statistic");
        console.log(basic_config_menu, project_config_menu)
        const p = {
            target: ev,
            items: [
                // {
                //     key: t("rmenu-open"),
                //     name: t("rmenu-open"),
                //     onClick: () => {
                //         open_file_location(item)
                //     }
                // },
                {
                    key: t("rmenu-copy-path"),
                    name: t("rmenu-copy-path"),
                    onClick: () => copy(item.abs_path)
                },
                {
                    key: t("rmenu-open-in-explorer"),
                    name: t("rmenu-open-in-explorer"),
                    onClick: () => {
                        open_file_location_in_explorer(item)
                    }
                },
                {
                    key: basic_config_menu,
                    name: basic_config_menu,
                    onClick: () => {
                        config_template_path(item, 'basic')
                    }
                },
                {
                    key: project_config_menu,
                    name: project_config_menu,
                    onClick: () => {
                        config_template_path(item, 'project')
                    }
                },
                {
                    key: t("rmenu-automation-basic_expense"),
                    name: t("rmenu-automation-basic_expense"),
                    onClick: () => {
                        excel_automation(item, 'basic')
                    }
                },
                {
                    key: t("rmenu-automation-project_statistic"),
                    name: t("rmenu-automation-project_statistic"),
                    onClick: () => {
                        excel_automation(item, 'project')
                    }
                },
                // {
                //     key: t("rmenu-open-in-terminal"),
                //     name: t("rmenu-open-in-terminal"),
                //     onClick: () => {
                //         open_file_location_in_terminal(item)
                //     }
                // },

            ],
            onDismiss: () => {
                setContextualMenuProps(undefined)
            }
        };

        if (index > -1) {
            setContextualMenuProps(p)
        }

        return false;
    }

    const classNames = mergeStyleSets({
        fileIconHeaderIcon: {
            padding: 0,
            fontSize: "12px"
        },
        fileIconCell: {
            textAlign: "center",
            selectors: {
                "&:before": {
                    content: ".",
                    display: "inline-block",
                    verticalAlign: "middle",
                    height: "100%",
                    width: "0px",
                    visibility: "hidden"
                }
            }
        },
        fileIconImg: {
            verticalAlign: "middle",
            maxHeight: "16px",
            maxWidth: "16px"
        },
        controlWrapper: {
            display: "flex",
            flexWrap: "wrap"
        },
        exampleToggle: {
            display: "inline-block",
            marginBottom: "10px",
            marginRight: "30px"
        },
        selectionDetails: {
            marginBottom: "20px"
        }
    });

    const columns = [
        {
            key: "column1",
            name: "File Type",
            className: classNames.fileIconCell,
            iconClassName: classNames.fileIconHeaderIcon,
            ariaLabel:
                "Column operations for File type, Press to sort on File type",
            iconName: "Page",
            isIconOnly: true,
            fieldName: "name",
            minWidth: 16,
            maxWidth: 16,

            onRender: (item) => {
                let isDir = R.prop('is_dir')(item);
                let name = R.prop("name")(item);
                const extSplit = R.split('.');
                let ext = R.last(extSplit(name));
                // let ext = "exe";
                // item.name
                return (
                    <div>
                        {
                            isDir ? <Icon {...getFileTypeIconProps({
                                    type: FileIconType.folder,
                                    size: 20,
                                    imageFileType: 'svg'
                                })} /> :
                                <Icon {...getFileTypeIconProps({extension: ext, size: 20, imageFileType: 'png'})} />
                        }
                    </div>
                );
            }
        },
        {
            key: "column2",
            name: t("name"),
            minWidth: 50,
            maxWidth: 200,
            isRowHeader: true,
            isResizable: true,
            data: "string",
            isPadded: true,
            onRender: (item) => {
                return <span>
                <Marker mark={tokenized} options={{className: "marker"}}>
       {item.name}
      </Marker>
                </span>;
            },
        },
        {
            key: "column3",
            name: t("last-modified"),
            fieldName: "mod_at",
            minWidth: 70,
            maxWidth: 90,
            isResizable: true,
            data: "number",
            onRender: (item) => {
                return <span>{tsFmt(item.mod_at)}</span>;
            },
            isPadded: true
        },
        {
            key: "column4",
            name: t("size"),
            fieldName: "size",
            minWidth: 40,
            maxWidth: 60,
            isSorted: false,
            isResizable: true,
            isCollapsible: true,
            data: "string",
            onRender: (item) => {
                return <span>{item.is_dir ? "-" : bytesToSize(item.size)}</span>;
            },
            isPadded: true
        },
        {
            key: "column5",
            name: t("path"),
            fieldName: "abs_path",
            minWidth: 70,
            isSorted: false,
            isResizable: true,
            isCollapsible: true,
            data: "number",
            onRender: (item) => {
                return <span>{item.abs_path}</span>;
            }
        }
    ];


    return (
        <div>
            <DetailsList
                onItemContextMenu={_onItemContextMenu}
                items={items}
                compact={true}
                columns={columns}
                selectionMode={SelectionMode.none}
                getKey={_getKey}
                setKey="none"
                layoutMode={DetailsListLayoutMode.justified}
                isHeaderVisible={true}
            />

            <ContextualMenu {...contextualMenuProps} />
        </div>
    );
}


export default Items;