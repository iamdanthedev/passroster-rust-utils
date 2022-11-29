import {parseRruleBetween, greet} from '../pkg/passroster_rust_utils'

describe('test', () => {
    it('should greet', () => {
        greet("Dan");
        expect(true).toBeTruthy();
    });

    it('should parse rrule', () => {
        const result = parseRruleBetween("DTSTART:20120201T093000Z\nRRULE:FREQ=DAILY", 60, "2020-02-01T09:30:00Z", "2020-02-02T09:30:00Z");
        expect(result).toHaveLength(4);
        
        const start1 = new Date(Number(result[0]));
        const end1 = new Date(Number(result[1]));
        const start2 = new Date(Number(result[2]));
        const end2 = new Date(Number(result[3]));
        
        expect(start1.toISOString()).toEqual("2020-02-01T09:30:00.000Z");
        expect(end1.toISOString()).toEqual("2020-02-01T10:30:00.000Z");
        expect(start2.toISOString()).toEqual("2020-02-02T09:30:00.000Z");
        expect(end2.toISOString()).toEqual("2020-02-02T10:30:00.000Z");
    });
});