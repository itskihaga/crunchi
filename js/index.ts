import * as member from "./member";
import * as room from "./room";
import * as record from "./record";
import {getYourId as getYourIdInner} from "./yourid";
import { connectFirestoreEmulator } from "firebase/firestore";
import { getStore } from "./firestore";
if(process.env.BUILD_MODE === "dev"){
    console.log("This is devmode!");
    const db = getStore();
    connectFirestoreEmulator(db,"localhost",3000);
}

export function syncMember(roomid:string,fn: (json:string) => void) : () => void {
    return member.syncMember(roomid,fn);
}

export function registerMember(roomid:string,name:string) {
    member.registerMember(roomid,name);
}

export function fetchMembers(roomid:string,fn: (json:string) => void) {
    member.fetchMembers(roomid,fn);
}

export async function createRoom(roomId:string,hostUserName:string,callback:(callback:string) => void) {
    room.createRoom(roomId,hostUserName).then(() => callback(roomId))
}

export function syncRoom(roomId:string,callback:(id:string | null) => void) : () => void {
    return room.syncRoom(roomId,callback)
}

export function startRoom(roomId:string) {
    room.startRoom(roomId);
}

export const pushRecord = (roomId:string,recordId:string,commandJson:string,resultJson:string) => {
    record.pushRecord(roomId,recordId,commandJson,resultJson);
}

export const syncRecordUpdate = (roomId:string,listener: (json:string) => void) : () => void => {
    return record.syncRecordUpdate(roomId,listener);
}

export const getYourId = (roomid: string) => getYourIdInner(roomid) 