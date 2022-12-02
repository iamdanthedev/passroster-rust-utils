import {parseBetween} from '../pkg/passroster_rust_utils';
import {rrulestr} from 'rrule';

describe('test', () => {
    it('should greet', () => {
        greet("Dan");
        expect(true).toBeTruthy();
    });

    it('should parse rrule', () => {
        const result = parseBetween("DTSTART:20120201T093000Z\nRRULE:FREQ=DAILY", 60, "2020-02-01T09:30:00Z", "2020-02-02T09:30:00Z");
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

describe('performance', () => {
    let rrules = [
        "DTSTART:20120201T093000Z\nRRULE:FREQ=HOURLY;INTERVAL=5",
        "DTSTART:20120201T093000Z\nFREQ=DAILY;INTERVAL=1",
        "DTSTART:20120201T093000Z\nFREQ=WEEKLY;BYDAY=SU,TU,TH,SA;INTERVAL=1"
    ]

    it('should parse rrule via rust', () => {
        for (const rrule of rrules) {
            const result = parseRruleBetween(rrule, 60, "2020-02-01T09:30:00Z", "2021-02-02T09:30:00Z");
            for (let i = 0; i < result.length; i += 2) {
                const start = new Date(Number(result[i]));
                const end = new Date(Number(result[i + 1]));
            }
        }
    });

    it('should parse rrule via js', () => {
        const after = new Date("2020-02-01T09:30:00Z");
        const before = new Date("2021-02-02T09:30:00Z");
        
        for (const rrule of rrules) {
            const rrule = rrulestr("DTSTART:20120201T093000Z\nRRULE:FREQ=HOURLY;INTERVAL=5");
            rrule.between(after, before);
        }
    });
});